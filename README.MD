# generate_keycode_by_string

Convert string to keycode for ydotool

> use the `fr` keyboard layout

>reference of the keycode ->
>https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h

## How to use ?

```bash
ydotool type 123azerty
>> &é"qwerty

ydotool key $(generate_keycode_by_string 123azerty)
>> 123azerty
```
