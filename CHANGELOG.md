### Chores
+ dependencies updated, [8bd0b9cbaa962320e2767152e2d5070e3d72b4d1]
### Features
+ download.sh added. [a9b887169490a43a2a39803f7713a1b9dc0a9a55]
### Refactors
+ camera in_use removed, [794e149245271e1a8749e0e05385cc20f3965725]

# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.1.0'>v0.1.0</a>
### 2023-01-24

### Chores
+ dependencies updated, [e1262252](https://github.com/mrjackwills/leafcast_pi/commit/e12622521d6a43e6b72bb79b4cc1e8dbe8328d5f), [f5ba7390](https://github.com/mrjackwills/leafcast_pi/commit/f5ba7390718c5f2730bfd483716da6467866e58a), [550a8c16](https://github.com/mrjackwills/leafcast_pi/commit/550a8c166494b1c68a47c83bb6d1cf3a98d414ba)

### Features
+ token request add timeout & useragent, [b0bcd91e](https://github.com/mrjackwills/leafcast_pi/commit/b0bcd91e64c31b51eb047610b4a6a87503e5a125)
+ Rotation enum, [facf97a3](https://github.com/mrjackwills/leafcast_pi/commit/facf97a347e08e20cd616a10e8dac1fce2478934)

### Fixes
+ replace depreciated base_64 methods, [dfd251e0](https://github.com/mrjackwills/leafcast_pi/commit/dfd251e057ec1748533724cd9604a6510ce683d7)

### Refactors
+ tracing_level into AppEnv, [2331971e](https://github.com/mrjackwills/leafcast_pi/commit/2331971e2c0b9e62e7e88a0ce11432d8b916bc8a)
+ is_connected removed, [d2cd2730](https://github.com/mrjackwills/leafcast_pi/commit/d2cd27305720ce5f57af44f75b01ce20d317db09)
+ Reaplace photograph `fn photograph<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Vec<u8>> + 'a + Send>>` with just a `async fn` & sleep [c549c28e](https://github.com/mrjackwills/leafcast_pi/commit/c549c28ee75bbfd66ec70a98940d1d78b063ddf8)


# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.0.7'>v0.0.7</a>
### 2022-12-16

### Chores
+ Rust 1.66 linting, [cfd4ac74](https://github.com/mrjackwills/leafcast_pi/commit/cfd4ac74795967b9d4b387c2326925229777b1d5)
+ dependencies updated, [dcc331e2](https://github.com/mrjackwills/leafcast_pi/commit/dcc331e2f6eff092b11449b514545d65286cd723)

### Features
+ feat: github action caching, [25e60789](https://github.com/mrjackwills/leafcast_pi/commit/25e6078905f33d072de5b1613b58b508ed94e116)
+ envTimezone get_offset(), [d3f28a67](https://github.com/mrjackwills/leafcast_pi/commit/d3f28a67971c31be9af55abc2b1cf23dbc0b7f85)

### Fixes
+ create_release.sh sed, [e950e3fc](https://github.com/mrjackwills/leafcast_pi/commit/e950e3fc60e0bf5872a4debb8ae575813e56bbe8)

# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.0.6'>v0.0.6</a>
### 2022-11-23

### Chores
+ github workflow use dtolnay/rust-toolchain, [ccc1af09](https://github.com/mrjackwills/leafcast_pi/commit/ccc1af09fd330ca8b248242e1023c53baa9b111e)
+ aggressive linting with rust 1.65.0, [c1964c55](https://github.com/mrjackwills/leafcast_pi/commit/c1964c55b3e53421d3ee9fd39002feb21bcc9a7f)
+ dependencies updated, [85e49fd6](https://github.com/mrjackwills/leafcast_pi/commit/85e49fd6778dece96e6c8d15b16b1daf2a784ed4)

### Docs
+ comments improved, dead code removed, [ b9cdce1e4602866acde0fecad45f3dd76fbfc0dd]

### Features
+ use EnvTimeZone struct to calculate current offset, [203edfff](https://github.com/mrjackwills/leafcast_pi/commit/203edfff109fc5a0fdf91aaff44b03f13f5f45e7)

### Fixes
+ create_release typo, [437b9f19](https://github.com/mrjackwills/leafcast_pi/commit/437b9f19805f843916b0130cc98feee79b9c3713)

# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.0.5'>v0.0.5</a>
### 2022-10-16

### Features
+ Use Image::webp conversion (requires libwebp to be installed), and execute in a tokio blocking thread, [59ad3ce6](https://github.com/mrjackwills/leafcast_pi/commit/59ad3ce6d83e90dce483ee870ce7262971bd189f),

### Chores
+ create_release v0.1.2, [b36b553c](https://github.com/mrjackwills/leafcast_pi/commit/b36b553cfef332b7fa339e7efd472fe51e456dd5),

# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.0.4'>v0.0.4</a>
### 2022-10-12

### Fixes
+ create_release.sh release flow fix?

# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.0.3'>v0.0.3</a>
### 2022-10-12

### Chores
+ cargo update, [1dbff86f](https://github.com/mrjackwills/leafcast_pi/commit/1dbff86f3f9e03541aaf622933f4b318516957e5),

### Fixes
+ create_release.sh Cargo.lock issue, [bc086031](https://github.com/mrjackwills/leafcast_pi/commit/bc08603195daeb3e190334906097f12bf763958c),
+ dev_container install cross, [5b8685ed](https://github.com/mrjackwills/leafcast_pi/commit/5b8685edd5c08c10e7018fd3aad145bd9a36e57d),

### Features
+ AutoClose websocket on ping, [57ed7294](https://github.com/mrjackwills/leafcast_pi/commit/57ed7294ac7c19bcfa450a809e0b473fecf865cf),

# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.0.2'>v0.0.2</a>
### 2022-10-10

### Chores
+ devcontainer updated, [2548aa9b](https://github.com/mrjackwills/leafcast_pi/commit/2548aa9bb1b83a2f96b3f0271dc02cd046774fa6),

### Features
+ create_release.sh build via cross-rs, [31256600](https://github.com/mrjackwills/leafcast_pi/commit/31256600ceafbab592c5333633cb1175b7c9d28a),
+ env check photo location exists, [71a9f639](https://github.com/mrjackwills/leafcast_pi/commit/71a9f6393a3fe7e870a3bd704b50becd8f08b921),
+ tracing, anyhow removed, [12d10cf1](https://github.com/mrjackwills/leafcast_pi/commit/12d10cf11b61fefdae39cf02ee8d7fda70bcc1e0),

### Fixes
+ use new staticPi protocol, [f3a6026d](https://github.com/mrjackwills/leafcast_pi/commit/f3a6026dd192925b6a242b1e5d1b9d7669b83102),

### Refactors
+ aggresive linting, [dbbe7469](https://github.com/mrjackwills/leafcast_pi/commit/dbbe74697dd5a35db191f40223281d00bf1ca286),
+ Camera.is_use use AtomicBool, [94f72bc5](https://github.com/mrjackwills/leafcast_pi/commit/94f72bc5c6a9f0c62acf3051c485ef41694d1108),
+ WS message handler, [4ee8b784](https://github.com/mrjackwills/leafcast_pi/commit/4ee8b784ec5682d7581a931138caa20a389f424c),
+ use global AppError, [91de86c0](https://github.com/mrjackwills/leafcast_pi/commit/91de86c06defd4ce0a76a138969c6c979277935f),

# <a href='https://github.com/mrjackwills/leafcast_pi/releases/tag/v0.0.1'>v0.0.1</a>
### 2022-05-31

### Features
+ init commit
