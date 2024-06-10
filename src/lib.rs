#[allow(dead_code)]
pub mod omc_mod {
  use wasm_bindgen::prelude::*;
  use std::collections::HashMap;
  use rand::Rng;
  use substring::Substring;

  // Some constants for the "transpiler"
  const OMC_COMMAND_COUNT: usize = 37;
  const OMC_ASCII_OFFSET: u32 = 32;

  // Some simple conversions
  fn internal_cu32to_string(n: u32) -> String {
    let out_char: char = char::from_u32(n + OMC_ASCII_OFFSET).expect(&format!("Failed to unpack \"{}\" as a charcode.", n));
    let out_str: String = String::from(out_char);
    return out_str;
  }
  fn internal_us2utf8(n: usize) -> String {
    return internal_cu32to_string(n as u32)
  }
  fn internal_n2utf8(n: &str) -> String {
    let char_code: u32 = u32::from_str_radix(n, 10).unwrap();
    return internal_cu32to_string(char_code);
  }

  // Blockly UID Generator (this will be used in OTAS later on)
  const _UID_CHARS: [&str; 90] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","0","1","2","3","4","5","6","7","8","9","`","~","!","@","#","$","%","^","&","*","(",")","_","+","-","=","{","}","|",";",":","<",">","?",",",".","/"," "];
  fn _uid() -> String {
    let mut i: i32 = 0;
    let mut soup: String = String::new();
    while i != 20 {
      let ingredient: &str = _UID_CHARS[rand::thread_rng().gen_range(0..=89)];
      soup.push_str(ingredient);
      i += 1;
    };
    return soup.to_owned().to_string();
  }

  // Get the mappings for functions and variable count
  pub fn get_oasm_map() -> [(String, u32); OMC_COMMAND_COUNT] {
    return [
      // V7 0-3 mappings in OMC v1.0 order
      // 0
      (String::from("pend"), 0),
      (String::from("penu"), 0),
      (String::from("pene"), 0),
      // 1
      (String::from("totv"), 1),
      (String::from("labl"), 1),
      (String::from("jump"), 1),
      (String::from("penc"), 1),
      (String::from("pens"), 1),
      (String::from("setx"), 1),
      (String::from("sety"), 1),
      (String::from("sinv"), 1),
      (String::from("cosv"), 1),
      (String::from("tanv"), 1),
      (String::from("sqrt"), 1),
      // 2
      (String::from("setp"), 2),
      (String::from("setv"), 2),
      (String::from("svto"), 2),
      (String::from("chav"), 2),
      (String::from("subv"), 2),
      (String::from("mulv"), 2),
      (String::from("divv"), 2),
      (String::from("modv"), 2),
      // 3
      (String::from("gthn"), 3),
      (String::from("lthn"), 3),
      (String::from("equl"), 3),
      (String::from("ngth"), 3),
      (String::from("nlth"), 3),
      (String::from("svtf"), 3),
      (String::from("svts"), 2), // This does take 3 it just reads from the bytes itself
      // V9 0-3 OCM v2 order
      // 0
      // No 0
      // 1
      (String::from("prnt"), 1),
      // 2
      (String::from("copy"), 2),
      (String::from("leng"), 2),
      // 3
      (String::from("letr"), 3),
      (String::from("join"), 3),
      (String::from("jnws"), 3),
      (String::from("neql"), 3),
      // Ranged
      (String::from("getd"), 2), // This can take 3-4 arguments
    ];
  }
  // Get the index of the "call_name" in the oasm_map
  fn oasm_map_call_index(oasm_map: [(String, u32); OMC_COMMAND_COUNT], call_name: String) -> usize {
    let mut i: usize = 0;
    while i < oasm_map.len() {
      let call: &(String, u32) = &oasm_map[i];
      if &call.0.to_owned() == &call_name {
        return i;
      }
      i += 1;
    }
    return oasm_map.len() + 1 as usize;
  }

  // |=======================|
  // |      OMC TO OASM      |
  // |=======================|

  // Convert OMC to OASM
  fn omc2oasm_internal_handle_command(code_ref: &mut Vec<char>, name: &String, argc: &u32) -> String {
    let skip_count: u32 = argc.to_owned();
    let code: &mut Vec<char> = code_ref;
    let passed_args: &mut Vec<u32> = &mut Vec::new();
    let mut i: i32 = (&skip_count).to_owned() as i32;
    let mut oasm: String = String::new();
    oasm.push_str(name);
    while i > 0 {
      let argc = (code[0] as u32) - (OMC_ASCII_OFFSET);
      passed_args.push(argc);
      let argu: &str = &(argc).to_string();
      oasm.push_str(" ");
      oasm.push_str(argu);
      code.remove(0);
      i -= 1;
    }
    match name.as_str() {
      // getd takes a string as an argument so we need to handle it in our own way
      "getd"=> {
        let mut rebuilt_oasm: String = String::new();
        rebuilt_oasm.push_str("getd ");
        match passed_args[0] as u32 {
          0=> {
            let var1: &str = &(code[0] as u32).to_string();
            code.remove(0);
            let var2 = &(code[0] as u32).to_string();
            code.remove(0);
            rebuilt_oasm.push_str("mousepos ");
            rebuilt_oasm.push_str(var1);
            rebuilt_oasm.push_str(" ");
            rebuilt_oasm.push_str(var2);
          },
          1=> {
            let var1: &str = &(code[0] as u32).to_string();
            code.remove(0);
            rebuilt_oasm.push_str("mouseclick ");
            rebuilt_oasm.push_str(var1);
          },
          2=> {
            let var1: &str = &(code[0] as u32).to_string();
            code.remove(0);
            rebuilt_oasm.push_str("line ");
            rebuilt_oasm.push_str(var1);
          },
          3=> {
            let var1: &str = &(code[0] as u32).to_string();
            code.remove(0);
            rebuilt_oasm.push_str("timer ");
            rebuilt_oasm.push_str(var1);
          },
          4=> {
            let var1: &str = &(code[0] as u32).to_string();
            code.remove(0);
            rebuilt_oasm.push_str("timestamp ");
            rebuilt_oasm.push_str(var1);
          },
          5=> {
            let key: &str = &String::from(code[0]);
            code.remove(0);
            let var1: &str = &(code[0] as u32).to_string();
            code.remove(0);
            rebuilt_oasm.push_str("key_");
            rebuilt_oasm.push_str(key);
            rebuilt_oasm.push_str(" ");
            rebuilt_oasm.push_str(var1);
          },
          _=> (),
        }
        oasm = rebuilt_oasm;
      }
      // The following are not real OASM commands
      // But instead intermediary commands for the "transpiler"
      "svtf"=> {
        let mut rebuilt_oasm: String = String::new();
        rebuilt_oasm.push_str("setv ");
        rebuilt_oasm.push_str(&passed_args[0].to_string());
        rebuilt_oasm.push_str(" ");
        rebuilt_oasm.push_str(&passed_args[1].to_string());
        rebuilt_oasm.push_str(".");
        rebuilt_oasm.push_str(&passed_args[2].to_string());
        oasm = rebuilt_oasm;
      },
      "svts"=> {
        let mut j: i32 = passed_args[1] as i32;
        let mut str_data: String = String::new();
        while j > 0 {
          str_data.push_str(code[0].to_string().as_str());
          code.remove(0);
          j -= 1;
        }

        let mut rebuilt_oasm: String = String::new();
        rebuilt_oasm.push_str("setv ");
        rebuilt_oasm.push_str(&passed_args[0].to_string());
        rebuilt_oasm.push_str(" ");
        rebuilt_oasm.push_str(&str_data);
        oasm = rebuilt_oasm;
      },
      // If we dont have a custom definition just dont care
      _=>(),
    }
    return oasm;
  }
  pub fn omc_to_oasm(oasm_map: [(String, u32); OMC_COMMAND_COUNT], omc: &str) -> String {
    let mut oasm: String = String::new();
    let mut code: Vec<char> = omc.chars().collect();
    while code.len() > (0 as usize) {
      let cmd_char: char = code[0];
      code.remove(0);
      // Get the command data
      let command_number: usize = ((cmd_char as u32) - OMC_ASCII_OFFSET) as usize;
      let command_tuple: &(String, u32) = &oasm_map[command_number];
      let oasm_line_bind: String = omc2oasm_internal_handle_command(code.as_mut(), &command_tuple.0, &command_tuple.1);
      // Push the command data and skip the next few characters
      oasm.push_str(&oasm_line_bind.to_owned());
      oasm.push_str("\n");
    }
    return oasm;
  }

  // |=======================|
  // |      OASM TO ONC      |
  // |=======================|
  fn oasm2_internal_handle_command(is_omc: bool, mut tokens: Vec<&str>, oasm_map: [(String, u32); OMC_COMMAND_COUNT], _vars: &mut HashMap<String, &str>) -> String {
    if is_omc {
      let mut omc: String = String::new();
      omc.push_str(&internal_us2utf8(oasm_map_call_index(oasm_map.to_owned(), tokens[0].to_string())));
      println!("{:?}",tokens);
      match tokens[0] {
        // Skip pend, penu and pene as they take no arguments
        // 1 No Data
        "totv"|"labl"|"jump"|"penc"|"pens"|"setx"|"sety"|
        "sinv"|"cosv"|"tanv"|"sqrt"|"prnt"=> {
          omc.push_str(&internal_n2utf8(&tokens[1]));
        },
        // 2 No Data
        "setp"|"svto"|"chav"|"subv"|"mulv"|"divv"|"modv"|
        "copy"|"leng"=> {
          omc.push_str(&internal_n2utf8(&tokens[1]));
          omc.push_str(&internal_n2utf8(&tokens[2]));
        },
        // 3 No Data
        "gthn"|"lthn"|"equl"|"ngth"|"nlth"|"svtf"|"letr"|
        "join"=> {
          omc.push_str(&internal_n2utf8(&tokens[1]));
          omc.push_str(&internal_n2utf8(&tokens[2]));
          omc.push_str(&internal_n2utf8(&tokens[3]));
        },
        // 1 Data
        // No 1 argc w/ data
        // 2 Data
        "setv"|"svts"|"getd"=> {
          match tokens[0] {
            "setv"=> {
              omc.push_str(&internal_n2utf8(&tokens[1]));
              omc.push_str(&internal_n2utf8(&tokens[2]));
            },
            "svts"=> {
              let str_lns: &str = *(&tokens.clone()[2]);
              let str_len: usize = u32::from_str_radix(str_lns, 10).unwrap() as usize;
              omc.push_str(&internal_n2utf8(&tokens[1]));
              tokens.remove(0); // Pop off the first 3 tokens
              tokens.remove(0);
              tokens.remove(0);
              // Now we can reconcat the string back to get the true string
              let arg_str: &str = &tokens.to_owned().join(" ");
              let str_data: &str = arg_str.substring(0 as usize, str_len);
              println!("String data: {}", str_data);
              omc.push_str(&internal_n2utf8(str_lns));
              omc.push_str(str_data);
            },
            _=> (),
          }
        },
        // 3 Data
        _=> (),
      }
      println!("{}",omc);
      return omc;
    } else {
      let mut _js: String = String::new();
      return _js;
    }
  }
  pub fn oasm_to_omc(oasm_map: [(String, u32); OMC_COMMAND_COUNT], oasm: &str) -> String {
    let mut omc: String = String::new();
    let mut _vars: HashMap<String, &str> = HashMap::new();
    for line in oasm.lines() {
      omc.push_str(&oasm2_internal_handle_command(true, line.trim_start().split(" ").collect(), oasm_map.to_owned(), &mut _vars));
    }
    return omc;
  }

  // |=======================|
  // |    OMC/OASM TO JS     |
  // |=======================|

  // OCM/OASM TO JS IS A WIP!!
  fn omc2js_internal_handle_command(code_ref: &mut Vec<char>, name: &String, _argc: &u32) -> String {
    let code: &mut Vec<char> = code_ref;
    let mut js: String = String::new();
    match name.as_str() {
      // 0
      "pend"=> {
        js.push_str("runtime.ext_pen.penDown({}, $util)");
      },
      "penu"=> {
        js.push_str("runtime.ext_pen.penUp({}, $util)");
      },
      "pene"=> {
        js.push_str("runtime.ext_pen.clear()");
      },
      // 1
      "totv"=> (), // We dont actually need this in JS
      "labl"=> (), // Skip label for now
      "jump"=> (), // Skip jump for now
      "penc"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("runtime.ext_pen.setPenColorToColor({{ COLOR: $vars['{}'] }}, $util);", var1));
      },
      "setx"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$util.target.setXY($vars['{}'], $util.target.y);", var1));
      },
      "sety"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$util.target.setXY($util.target.x, $vars['{}']);", var1));
      },
      "sinv"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$vars['{var1}'] = Math.sin($vars['{var1}']);"));
      },
      "cosv"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$vars['{var1}'] = Math.cos($vars['{var1}']);"));
      },
      "tanv"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$vars['{var1}'] = Math.tan($vars['{var1}']);"));
      },
      "sqrt"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$vars['{var1}'] = Math.sqrt($vars['{var1}']);"));
      },
      // 2
      "setp"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        let var2: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$util.target.setXY($vars['{}'], $vars['{}']);", var1, var2));
      },
      // 3
      "svtf"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        let var2: &str = &(code[0] as u32).to_string();
        code.remove(0);
        let var3: &str = &(code[0] as u32).to_string();
        code.remove(0);
        js.push_str(&format!("$vars['{}'] = {}.{};", var1, var2, var3));
      },
      "svts"=> {
        let var1: &str = &(code[0] as u32).to_string();
        code.remove(0);
        let str_len: u32 = 4; // <-------------------------------------- GOTTA FIX THIS!!!!!!
        println!("strlen {:?}", str_len);
        code.remove(0);
        let mut var2: String = String::new();
        let mut i: u32 = 0;
        while i < str_len {
          var2.push(code[i as usize]);
          i += 1;
          if i == str_len {
            break;
          }
        }
        js.push_str(&format!("$vars['{}'] = '{}';", var1, var2.replace("\\", "\\\\").replace("'", "\\'")));
      },
      _=> (),
    }
    return js;
  }
  pub fn omc_to_js(oasm_map: [(String, u32); OMC_COMMAND_COUNT], omc: &str) -> String {
    let mut js: String = String::new();
    js.push_str("(function(runtime) {
      const $util = {target: runtime.targets[1]}, $vars = {};");
    let mut code: Vec<char> = omc.chars().collect();
    while code.len() > (0 as usize) {
      let cmd_char: char = code[0];
      code.remove(0);
      // Get the command data
      let command_number: usize = ((cmd_char as u32) - OMC_ASCII_OFFSET) as usize;
      let command_tuple: &(String, u32) = &oasm_map[command_number];
      let js_line_bind: String = omc2js_internal_handle_command(code.as_mut(), &command_tuple.0, &command_tuple.1);
      // Push the command data and skip the next few characters
      js.push_str(&js_line_bind.to_owned());
      js.push_str("\n");
    }
    js.push_str("\n})();");
    return js;
  }
  pub fn oasm_to_js(oasm_map: [(String, u32); OMC_COMMAND_COUNT], oasm: &str) -> String {
    let omc = oasm_to_omc(oasm_map.to_owned(), oasm);
    return omc_to_js(oasm_map.to_owned(), &omc);
  }

  // |============================|
  // |       Entry Functions      |
  // |============================|
  #[wasm_bindgen]
  pub fn oasm2omc(oasm: &str) -> String {
    let oasm_map: [(String, u32); OMC_COMMAND_COUNT] = get_oasm_map().to_owned();
    return oasm_to_omc(oasm_map.to_owned(), oasm);
  }
  // #[wasm_bindgen]
  fn oasm2js(oasm: &str) -> String {
    let oasm_map: [(String, u32); OMC_COMMAND_COUNT] = get_oasm_map().to_owned();
    return oasm_to_js(oasm_map.to_owned(), oasm);
  }
  #[wasm_bindgen]
  pub fn omc2oasm(omc: &str) -> String {
    let oasm_map: [(String, u32); OMC_COMMAND_COUNT] = get_oasm_map().to_owned();
    return omc_to_oasm(oasm_map.to_owned(), omc);
  }
  // #[wasm_bindgen]
  fn omc2js(omc: &str) -> String {
    let oasm_map: [(String, u32); OMC_COMMAND_COUNT] = get_oasm_map().to_owned();
    return omc_to_js(oasm_map.to_owned(), omc);
  }
}