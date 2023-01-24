### 2023-01-24

### Chores
+ dependencies updated, [e12622521d6a43e6b72bb79b4cc1e8dbe8328d5f], [f5ba7390718c5f2730bfd483716da6467866e58a], [550a8c166494b1c68a47c83bb6d1cf3a98d414ba]

### Features
+ token request add timeout & useragent, [b0bcd91e64c31b51eb047610b4a6a87503e5a125]
+ Rotation enum, [facf97a347e08e20cd616a10e8dac1fce2478934]

### Fixes
+ replace depreciated base_64 methods, [dfd251e057ec1748533724cd9604a6510ce683d7]

### Refactors
+ tracing_level into AppEnv, [2331971e2c0b9e62e7e88a0ce11432d8b916bc8a]
+ is_connected removed, [d2cd27305720ce5f57af44f75b01ce20d317db09]
+ Reaplace photograph `fn photograph<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Vec<u8>> + 'a + Send>>` with just a `async fn` & sleep [c549c28ee75bbfd66ec70a98940d1d78b063ddf8]



see <a href='https://github.com/mrjackwills/leafcast_pi/blob/main/CHANGELOG.md'>CHANGELOG.md</a> for more details
