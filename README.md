# zbusctl

A command-line utility for making D-Bus method calls. Written in Rust 💪.

## Usage

```bash
zbusctl call [OPTIONS] --service <SERVICE> --object <PATH> --interface <INTERFACE> --method <METHOD> [ARGS...]
```

### Arguments

- `-s, --service <SERVICE>`: D-Bus service name (e.g., `org.freedesktop.NetworkManager`)
- `-o, --object <PATH>`: D-Bus object path (e.g., `/org/freedesktop/NetworkManager`)
- `-i, --interface <INTERFACE>`: D-Bus interface name (e.g., `org.freedesktop.NetworkManager`)
- `-m, --method <METHOD>`: D-Bus method name (e.g., `GetDevices`)
- `[ARGS...]`: Method arguments in `type:value` format (optional)
- `--system`: Use system bus instead of session bus (optional)

### Supported Argument Types

Arguments are specified using `type:value` format. Supported types include:

#### Basic Types
- `string` - string value
- `int32` - 32-bit signed integer
- `uint32` - 32-bit unsigned integer
- `int64` - 64-bit signed integer
- `uint64` - 64-bit unsigned integer
- `int16` - 16-bit signed integer
- `uint16` - 16-bit unsigned integer
- `byte` - 8-bit unsigned integer (0-255)
- `double` - double-precision floating point
- `boolean` or `bool` - boolean value (true/false)
- `objpath` - D-Bus object path
- `signature` - D-Bus type signature

#### Array Types
- `array:<element_type>:<comma_separated_values>` - array of elements

Supported array element types:
- `array:int32:1,2,3,4` - array of 32-bit signed integers
- `array:uint32:1,2,3,4` - array of 32-bit unsigned integers
- `array:int64:1,2,3,4` - array of 64-bit signed integers
- `array:uint64:1,2,3,4` - array of 64-bit unsigned integers
- `array:int16:1,2,3,4` - array of 16-bit signed integers
- `array:uint16:1,2,3,4` - array of 16-bit unsigned integers
- `array:byte:1,2,3,4` - array of bytes (0-255)
- `array:double:1.0,2.5,3.14` - array of double-precision floats
- `array:boolean:true,false,true` - array of boolean values
- `array:string:hello,world,test` - array of strings
- `array:objpath:/path1,/path2` - array of object paths
- `array:signature:s,i,d` - array of type signatures

#### Dictionary Types
- `dict:<key_type>:<value_type>:<comma_separated_pairs>` - dictionary/map of key-value pairs

Supported dictionary examples:
- `dict:string:int32:"one",1,"two",2,"three",3` - string keys to int32 values
- `dict:string:string:"name","John","city","NYC"` - string keys to string values
- `dict:string:boolean:"enabled",true,"debug",false` - string keys to boolean values

Supported key types: `string`
Supported value types: `string`, `int32`, `uint32`, `int64`, `uint64`, `int16`, `uint16`, `byte`, `double`, `boolean`/`bool`

### Examples

#### 1. Call a method with no arguments

```bash
zbusctl call --system \
             -s org.freedesktop.NetworkManager \
             -o /org/freedesktop/NetworkManager \
             -i org.freedesktop.NetworkManager \
             -m GetDevices
```

#### 2. Call a method with a string argument

```bash
zbusctl call -s org.example.Service \
             -o /org/example/Object \
             -i org.example.Interface \
             -m SetName \
             string:"Hello World"
```

#### 3. Call a method with multiple arguments

```bash
zbusctl call -s org.example.Service \
             -o /org/example/Object \
             -i org.example.Interface \
             -m SetValues \
             string:"Hello" int32:42
```

#### 4. Call a method with an array argument

```bash
zbusctl call -s org.example.Service \
             -o /org/example/Object \
             -i org.example.Interface \
             -m ProcessNumbers \
             array:int32:1,2,3,4,5
```

#### 5. Call a method with mixed arguments including arrays

```bash
zbusctl call -s org.example.Service \
             -o /org/example/Object \
             -i org.example.Interface \
             -m ProcessData \
             string:"config" array:string:opt1,opt2,opt3 int32:42
```

#### 6. Use system bus instead of session bus

```bash
zbusctl call --system \
             -s org.freedesktop.systemd1 \
             -o /org/freedesktop/systemd1 \
             -i org.freedesktop.systemd1.Manager \
             -m ListUnits
```

#### 6. Call a method with different data types

```bash
zbusctl call -s org.example.Service \
             -o /org/example/Object \
             -i org.example.Interface \
             -m ProcessData \
             boolean:true double:3.14159 uint32:12345
```

#### 7. Call a method with a dictionary argument

```bash
zbusctl call -s org.example.Service \
             -o /org/example/Object \
             -i org.example.Interface \
             -m SetConfiguration \
             dict:string:int32:"timeout",30,"retries",3,"port",8080
```

## Building

```bash
cargo build --release
```

## Installation

```bash
cargo install --path .
```

## License

This project is licensed under the MIT License.
