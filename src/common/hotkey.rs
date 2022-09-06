use device_query::{DeviceQuery, DeviceState};

pub use device_query::Keycode;

#[derive(Debug, Clone)]
pub struct Hotkey {
    keys: Vec<Keycode>,
    callback: fn(),
}

impl Hotkey {
    pub fn new(keys: Vec<Keycode>, callback: fn()) -> Self {
        Self { keys, callback }
    }
}

pub struct HotkeyManager {
    hot_keys: Vec<Hotkey>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        let hm = Self {
            hot_keys: Vec::new(),
        };
        hm
    }

    fn match_keys(keys: &Vec<Keycode>, hot_keys: &Vec<Keycode>) -> bool {
        let mut matched = true;
        for key in hot_keys.iter() {
            if !keys.contains(key) {
                matched = false;
                break;
            }
        }
        matched
    }

    fn check_hooks(keys: Vec<Keycode>, hot_keys: &Vec<Hotkey>) {
        for hk in hot_keys {
            if Self::match_keys(&keys, &hk.keys) {
                (hk.callback)();
            }
        }
    }

    pub fn listen(&self) {
        let hot_keys = self.hot_keys.clone();
        // 开启新线程 用于监听键盘事件
        std::thread::spawn(move || {
            let mut keys: Vec<Keycode> = Vec::new();
            let device_state = DeviceState::new();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(100));
                let keys_down = device_state.get_keys();
                if keys_down.ne(&keys) {
                    keys = keys_down.clone();
                    Self::check_hooks(keys_down, &hot_keys);
                }
            }
        });
    }

    pub fn register(&mut self, hotkey: Hotkey) {
        self.hot_keys.push(hotkey.clone());
    }
}
