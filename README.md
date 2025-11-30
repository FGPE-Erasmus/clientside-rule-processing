# Clientside Rule Processing

Clientside application used for event processing.

## Features

- rock solid and fast (thanks to Rust)
- compiled to WASM, efficient and portable format
- flexible parsing with `pest` parser

## Installation & usage

Prerequisites:
- [Rust](https://www.rust-lang.org/learn/get-started)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (or just `cargo install wasm-pack`)

Building:

    1. Open project directory in terminal
    2. Navigate to /browser module
    3. Execute build command `wasm-pack build --target web`
    4. Enjoy results from `/pkg` directory

Testing:
    
    1. Open project directory in terminal
    2. Execute command `cargo test`

Usage:

The main file you will interact with is located in /pkg directory and is named `browser.js`.
It exports one function `process_event` as documented in the API section. 
It processes an event created by user, updates and returns their context (game state) along with a result (reward). 
The function is asynchronous, so expect a `Promise`.

Sample usage from HTML file can be seen in `/browser/index.html`

## API

Origin:

`browser.js`

Function:

name: `process_event`

type: `async` (`Promise`)

argument: `dictionary`

sample argument format:
```
{
   "event":{
      "parts":{
         "player":{
            "NumberBased":1
         },
         "on":{
            "DateBased":"2024-10-27"
         },
         "at":{
            "TimeBased":"10:30:00"
         },
         "result":{
            "NumberBased":100
         }
         // other fields
      }
   },
   "game_state":"",
   "simple_rules_str":"simple_rule: player 1 on 2020.01.01..2026.01.01 at 8:30..23:30 achieving 100 repeat +",
   "compound_rules_str":"",
   "rule_results_str":"simple_rule -> msg nicely_done_simple_rule"
}
```

returns:

`dictionary` if succeeded, `undefined` otherwise

sample success:

```
{
   "game_state":"...",
   "results":[
      [
         "Message",
         [
            "nicely_done_simple_rule"
         ]
      ]
   ]
}
```


*   Acknowledgments

<table cellspacing="0" cellpadding="0" border=0>
<tr border=0>
<td border=0>

This software has been developed as a part of the FGPE++ Gamified Programming Learning at Scale (https://fgpeplus2.usz.edu.pl/) project which was co-funded by the European Union.

</td>
<td border=0>

![Framework for Gamified Programming Education project](logo_FGPE.jpg) ![Erasmus+](logo_erasmus.jpg)

</td>
</tr>
</table>

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
