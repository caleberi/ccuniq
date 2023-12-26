use crate::constant;

#[derive(Debug)]
pub struct Config<'a> {
    // Precede each output line with the count of the number of times the line occurred in the input, followed by a single space.
    pub count: bool,
    // Output a single copy of each line that is repeated in the input.
    pub repeated: bool,
    // Output all lines that are repeated (like -d, but each copy of the repeated line is written).
    // The optional septype argument controls how to separate groups of repeated lines
    // in the output; it must be one of the following values:
    //  none      Do not separate groups of lines (this is the default).
    //  prepend   Output an empty line before each group of lines.
    //  separate  Output an empty line after each group of lines.
    pub all_repeated: constant::RepeatedTypeOpt<'a>,
    // Ignore the first num fields in each input line when doing comparisons.
    // A field is a string of non-blank characters separated from adjacent fields by blanks.
    // Field numbers are one based, i.e., the first field is field one.
    pub num: bool,
    // Case insensitive comparison of lines.
    pub ignore_case: bool,
    // Ignore the first chars characters in each input line when doing comparisons.
    // If specified in conjunction with the -f, --unique option, the first chars characters after the
    // first num fields will be ignored.  Character numbers are one based, i.e., the first character is character one.
    pub skip_chars: bool,
    // Only output lines that are not repeated in the input.
    pub unique: bool,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
}

impl<'a> Config<'a> {
    pub fn new(args: std::env::Args) -> Config<'a> {
        Config::build_config(args.collect())
    }

    fn build_config(args: Vec<String>) -> Config<'a> {
        let mut cfg: Config = Config {
            count: false,
            repeated: false,
            all_repeated: constant::RepeatedTypeOpt::Value(constant::RepeatedOpt {
                condition: false,
                value: "none",
            }),
            num: false,
            ignore_case: false,
            skip_chars: false,
            unique: false,
            input_file: None,
            output_file: None,
        };

        if args.len() == 0 {
            return cfg;
        }

        Config::parse_config_from_args(&args, &mut cfg);

        cfg
    }

    fn parse_config_io_result(
        index: usize,
        args: &Vec<String>,
        is_input_file: &mut bool,
        cfg: &mut Config,
    ) {
        if index >= args.len() - 2 && index != 0 {
            let file_path = args.get(index).unwrap();
            if file_path != "" {
                if !file_path.starts_with("-") && !*is_input_file {
                    cfg.input_file = Some(file_path.clone());
                    *is_input_file = !*is_input_file;
                } else if file_path != "" && !file_path.starts_with("-") && *is_input_file {
                    cfg.output_file = Some(file_path.clone());
                }
            }
        }
    }

    fn parse_config_from_args(args: &Vec<String>, cfg: &mut Config) {
        let mut is_input_file: bool = false;
        if args.len() < 2 {
            return;
        }
        // edge case for eg uniq test.txt
        if args.len() == 2 {
            let file_path = args.get(1).unwrap();
            if !file_path.starts_with("-") {
                cfg.input_file = Some(file_path.clone());
            }
        }
        for mut index in 0..args.len() {
            // parse the input and output files
            Config::parse_config_io_result(index, args, &mut is_input_file, cfg);
            let arg = args.get(index).unwrap();
            match arg.as_str() {
                "-c" => cfg.count = true,
                "-d" => cfg.repeated = true,
                "-D" => {
                    index += 1; // get the value
                    let next_arg = args.get(index);
                    if !next_arg.is_none() {
                        let option = next_arg.unwrap().clone();
                        cfg.all_repeated = match option.as_str() {
                            "none" => constant::NONE_REPEATED_TYPE_OPT,
                            "prepend" => constant::PREPEND_REPEATED_TYPE_OPT,
                            "seperate" => constant::SEPERATE_REPEATED_TYPE_OPT,
                            _ => constant::NONE_REPEATED_TYPE_OPT,
                        }
                    }
                }
                "-f" => cfg.num = true,
                "-i" => cfg.ignore_case = true,
                "-s" => cfg.skip_chars = true,
                "-u" => cfg.unique = true,
                _ => (),
            }
        }
    }

    pub fn get_flag_status(&self) -> bool {
        let all_repeated_flag_status = match &self.all_repeated {
            constant::RepeatedTypeOpt::Value(repeated_opt) => repeated_opt.condition,
        };
        return self.count
            || self.num
            || self.ignore_case
            || self.repeated
            || self.skip_chars
            || self.unique
            || all_repeated_flag_status;
    }
}
