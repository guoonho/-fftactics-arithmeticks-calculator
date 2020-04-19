struct Unit {
    is_target: bool,
    ct: i8,
    exp: i8,
    level: i8,
    height: f32
}

pub fn validate(_input: &String) -> bool {
    true
}

pub fn calculate(_input: &String) -> String {
    let s = _input.split(",");
    let mut result = String::from("");
    let mut units = Vec::<Unit>::new();

    for elem in s {
        let cur = elem.split("/");
        let vec: Vec<&str> = cur.collect();

        let unit = Unit {
            is_target: vec[0].parse::<bool>().unwrap(),
            ct: vec[1].parse::<i8>().unwrap(),
            exp: vec[2].parse::<i8>().unwrap(),
            level: vec[3].parse::<i8>().unwrap(),
            height: vec[4].parse::<f32>().unwrap()
        };
        units.push(unit);
    }

    let ct_results = check_ct(&units);
    let exp_results = check_exp(&units);
    let level_results = check_level(&units);
    let height_results = check_height(&units);

    result.push_str(&ct_results);
    result.push_str(&exp_results);
    result.push_str(&level_results);
    result.push_str(&height_results);

    result
}

fn check_ct(_units: &Vec::<Unit>) -> String{
    let mut result = String::from("");
    let mut flag_prime = true;
    let mut flag_five = true;
    let mut flag_four = true;
    let mut flag_three = true;

    for unit in _units {
        if unit.is_target {
            if !is_prime(&unit.ct) {
                flag_prime = false;
            }
            if !is_mult_5(&unit.ct) {
                flag_five = false;
            }
            if !is_mult_4(&unit.ct) {
                flag_four = false;
            }
            if !is_mult_3(&unit.ct) {
                flag_three = false;
            }
        }
    }

    if flag_prime {
        result.push_str("ct/prime ");
    }
    if flag_five {
        result.push_str("ct/5 ");
    }
    if flag_four {
        result.push_str("ct/4 ");
    }
    if flag_three {
        result.push_str("ct/3 ");
    }

    result
}

fn check_exp(_units: &Vec::<Unit>) -> String{
    let mut result = String::from("");
    let mut flag_prime = true;
    let mut flag_five = true;
    let mut flag_four = true;
    let mut flag_three = true;

    for unit in _units {
        if unit.is_target {
            if !is_prime(&unit.exp) {
                flag_prime = false;
            }
            if !is_mult_5(&unit.exp) {
                flag_five = false;
            }
            if !is_mult_4(&unit.exp) {
                flag_four = false;
            }
            if !is_mult_3(&unit.exp) {
                flag_three = false;
            }
        }
    }

    if flag_prime {
        result.push_str("exp/prime ");
    }
    if flag_five {
        result.push_str("exp/5 ");
    }
    if flag_four {
        result.push_str("exp/4 ");
    }
    if flag_three {
        result.push_str("exp/3 ");
    }

    result
}

fn check_level(_units: &Vec::<Unit>) -> String{
    let mut result = String::from("");
    let mut flag_prime = true;
    let mut flag_five = true;
    let mut flag_four = true;
    let mut flag_three = true;

    for unit in _units {
        if unit.is_target {
            if !is_prime(&unit.level) {
                flag_prime = false;
            }
            if !is_mult_5(&unit.level) {
                flag_five = false;
            }
            if !is_mult_4(&unit.level) {
                flag_four = false;
            }
            if !is_mult_3(&unit.level) {
                flag_three = false;
            }
        }
    }

    if flag_prime {
        result.push_str("level/prime ");
    }
    if flag_five {
        result.push_str("level/5 ");
    }
    if flag_four {
        result.push_str("level/4 ");
    }
    if flag_three {
        result.push_str("level/3 ");
    }

    result
}

fn check_height(_units: &Vec::<Unit>) -> String{
    let mut result = String::from("");
    let mut flag_prime = true;
    let mut flag_five = true;
    let mut flag_four = true;
    let mut flag_three = true;

    for unit in _units {
        if unit.is_target {
            if unit.height.fract() != 0.0 {
                return result;
            }

            let height = unit.height.trunc() as i8;

            if !is_prime(&height) {
                flag_prime = false;
            }
            if !is_mult_5(&height) {
                flag_five = false;
            }
            if !is_mult_4(&height) {
                flag_four = false;
            }
            if !is_mult_3(&height) {
                flag_three = false;
            }
        }
    }

    if flag_prime {
        result.push_str("height/prime ");
    }
    if flag_five {
        result.push_str("height/5 ");
    }
    if flag_four {
        result.push_str("height/4 ");
    }
    if flag_three {
        result.push_str("height/3 ");
    }

    result
}

fn is_prime(_input: &i8) -> bool {
    let mut flag = true;
    if _input == &1 || _input == &4 {
        flag = false;
    }

    let m = _input / 2;

    for x in 2..m {
        if _input % x == 0 {
            return false;
        }
    }

    flag
}

fn is_mult_3(_input: &i8) -> bool {
    if _input % 3 == 0 {
        true
    }
    else {
        false
    }
}

fn is_mult_4(_input: &i8) -> bool {
    if _input % 4 == 0 {
        true
    }
    else {
        false
    }
}

fn is_mult_5(_input: &i8) -> bool {
    if _input % 5 == 0 {
        true
    }
    else {
        false
    }
}

#[test]
fn test_validate() {
    let test = "true/2/12/4/3,false/2/4/6/3,true/2/14/4/6,true/11/77/5/4".to_string();
    let result = calculate(&test);

    assert!(result.contains("ct/prime "));
}
