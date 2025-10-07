use alloc::string::{String, ToString};

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    // コンストラクタ　インスタンスを作成する関数
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    // 以降はメソッド

    // Resultの右側がparse成功時の値、左側が失敗時のエラーメッセージ
    // &mut selfは可変参照で、インスタンス自体を変更することができる
    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }
    }

    // httpスキーマを省略していないかの判定
    fn is_http(&mut self) -> bool {
        self.url.contains("http://")
    }

    // host部分を抽出するメソッド
    fn extract_host(&self) -> String {
        let url_patrs: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // port番号が指定されてたら、:の前までをhostとして抽出
        if let Some(index) = url_parts[0].find(":") {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    // port番号を抽出するメソッド
    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // port番号が指定されてたら、:の後ろまでをportとして抽出
        if let Some(index) = url_parts[0].find(":") {
            url_parts[0][index + 1..].to_string()
        } else {
            // httpのデフォルトポートは80番
            "80".to_string()
        }
    }
}
