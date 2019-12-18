#[cfg(test)]
mod tests_mod {
    use super::super::{Config, RSH};
    use std::{fs, path::Path};
    #[test]
    fn simple() {
        for file in fs::read_dir(Path::new("./src/tests")).unwrap() {
            let path = file.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "sh" {
                    RSH::new(Config {
                        input: None,
                        script_path: Some(path.to_str().unwrap().to_string()),
                    })
                    .run()
                    .unwrap();
                }
            }
        }
    }

    use super::super::exec::Program;
    use super::super::parsing::*;

    #[test]
    fn zsh_compliance() {
        let cmds = vec![
            //basic
            "echo test",
            "ls",
            "ls /",
            "ls .",
            //bool
            "ls && ls",
            "ls || ls",
            "ls && ls && ls",
            "ls && ls || ls",
            "ls || ls && ls",
            "ls || ls || ls",
            //seq
            "ls; ls; ls",
            "ls; ls; ls;",
            //pipe
            "echo \"/\" | cat",
            "dmesg | grep i",
            //parenthesis
            "(ls && ls) || ls",
            "(ls || ls) && ls",
            "ls && (ls || ls)",
            "ls || (ls && ls)",
            // redir
            // "ls > /tmp/lol",
            // backticks
            "ls `echo /`",
            "echo `ls`",
            "ls `find src`",
            // Interpolation
            "echo $USER",
            "echo ${USER}",
            "echo \"$(whoami)\"",
            // complex,
            "ls; ls || ls || ls asd && ls | cat",
            "ls; ls && ls && ls asd && ls | cat || true",
        ];

        let cmds = cmds.iter().map(|x| x.to_string()).collect::<Vec<_>>();

        for cmd in cmds {
            let mut prog1 = Program::new(CommandRaw::new(
                "zsh".to_string(),
                vec!["-c".to_string(), cmd.clone()],
            ));

            let mut prog2 = Program::new(CommandRaw::new(
                "rsh".to_string(),
                vec!["-e".to_string(), cmd.clone()],
            ));

            let out1 = prog1.prog.output().unwrap();
            let out2 = prog2.prog.output().unwrap();

            assert_eq!(out1, out2);
        }
    }
}
