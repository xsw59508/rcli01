use anyhow::{anyhow, Result};
use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

/// 密码生成配置
#[derive(Debug, Clone)]
struct PasswordConfig {
    length: u8,
    use_upper: bool,
    use_lower: bool,
    use_number: bool,
    use_symbol: bool,
}

/// 字符集构建器
struct CharSetBuilder {
    chars: Vec<u8>,
    required_chars: Vec<u8>,
}

impl CharSetBuilder {
    fn new() -> Self {
        Self {
            chars: Vec::new(),
            required_chars: Vec::new(),
        }
    }

    fn add_charset(&mut self, charset: &[u8], enabled: bool) -> Result<()> {
        if !enabled {
            return Ok(());
        }

        if charset.is_empty() {
            return Err(anyhow!("Character set cannot be empty"));
        }

        self.chars.extend_from_slice(charset);

        // 添加一个必需字符以确保每种选中的类型至少出现一次
        let random_char = charset
            .choose(&mut rand::rng())
            .ok_or_else(|| anyhow!("Failed to select random character"))?;
        self.required_chars.push(*random_char);

        Ok(())
    }

    fn build(self) -> (Vec<u8>, Vec<u8>) {
        (self.chars, self.required_chars)
    }
}

/// 验证密码生成参数
fn validate_password_config(config: &PasswordConfig) -> Result<()> {
    if config.length == 0 {
        return Err(anyhow!("Password length cannot be zero"));
    }

    if config.length > 128 {
        return Err(anyhow!("Password length cannot exceed 128 characters"));
    }

    if !config.use_upper && !config.use_lower && !config.use_number && !config.use_symbol {
        return Err(anyhow!("At least one character type must be selected"));
    }

    // 检查长度是否足够包含所有必需的字符类型
    let required_types = [
        config.use_upper,
        config.use_lower,
        config.use_number,
        config.use_symbol,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    if config.length < required_types as u8 {
        return Err(anyhow!(
            "Password length {} is too short for {} required character types",
            config.length,
            required_types
        ));
    }

    Ok(())
}

/// 生成随机密码
fn generate_password(config: &PasswordConfig) -> Result<String> {
    let mut charset_builder = CharSetBuilder::new();

    charset_builder.add_charset(UPPER, config.use_upper)?;
    charset_builder.add_charset(LOWER, config.use_lower)?;
    charset_builder.add_charset(NUMBER, config.use_number)?;
    charset_builder.add_charset(SYMBOL, config.use_symbol)?;

    let (available_chars, mut required_chars) = charset_builder.build();

    if available_chars.is_empty() {
        return Err(anyhow!("No available characters for password generation"));
    }

    let mut rng = rand::rng();
    let mut password_chars = Vec::with_capacity(config.length as usize);

    // 添加必需字符
    password_chars.append(&mut required_chars);

    // 填充剩余位置
    let remaining_length = config.length - password_chars.len() as u8;
    for _ in 0..remaining_length {
        let random_char = available_chars
            .choose(&mut rng)
            .ok_or_else(|| anyhow!("Failed to select random character from available set"))?;
        password_chars.push(*random_char);
    }

    // 打乱字符顺序
    password_chars.shuffle(&mut rng);

    // 转换为字符串
    String::from_utf8(password_chars)
        .map_err(|e| anyhow!("Failed to convert password to string: {}", e))
}

/// 评估并显示密码强度
fn evaluate_password_strength(password: &str) {
    let estimate = zxcvbn(password, &[]);
    let score = estimate.score();
    eprintln!("Password strength: {}", score);

    // 提供更详细的反馈 - Score 是一个枚举类型，使用模式匹配
    match score {
        zxcvbn::Score::Zero => {
            eprintln!("Weak password - consider adding more characters or complexity")
        }
        zxcvbn::Score::One => {
            eprintln!("Weak password - consider adding more characters or complexity")
        }
        zxcvbn::Score::Two => eprintln!("Moderate password strength"),
        zxcvbn::Score::Three => eprintln!("Moderate password strength"),
        zxcvbn::Score::Four => eprintln!("Strong password"),
        _ => eprintln!("Unknown password strength score"),
    }

    if let Some(feedback) = estimate.feedback() {
        if let Some(warning) = feedback.warning() {
            eprintln!("Warning: {}", warning);
        }

        if !feedback.suggestions().is_empty() {
            eprintln!("Suggestions:");
            for suggestion in feedback.suggestions() {
                eprintln!("  - {}", suggestion);
            }
        }
    }
}

/// 处理密码生成请求
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<()> {
    let config = PasswordConfig {
        length,
        use_upper: upper,
        use_lower: lower,
        use_number: number,
        use_symbol: symbol,
    };

    // 验证配置
    validate_password_config(&config)?;

    // 生成密码
    let password = generate_password(&config)?;

    // 输出结果
    println!("{}", password);
    evaluate_password_strength(&password);

    Ok(())
}
