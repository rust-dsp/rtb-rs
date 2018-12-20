use std::sync::Arc;

pub struct XHandle {
    conn: Arc<xcb::Connection>,
    screen_num: i32,
}

impl XHandle {
    pub fn new() -> Self {
        let (conn, screen_num) = xcb::base::Connection::connect(None).unwrap();

        Self {
            conn: Arc::new(conn),
            screen_num,
        }
    }

    pub fn conn(&self) -> Arc<xcb::base::Connection> {
        self.conn.clone()
    }

    pub fn screen_num(&self) -> i32 {
        self.screen_num
    }
}