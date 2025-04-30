use Ztor::{Lexer, ProgramParser};

//Correct

#[test]
fn test_base() {
    let source = "    program prueba1;  
                            var a,b : int;
                            var c,d : float;
                            void func1(a : int, b : int)[
                                var c,d : float;
                                {
                                    print(a, b, ((c + d) + 5));
                                }
                            ];
                            main
                            {
                                if(a > (c * d) ) do
                                {
                                    func1(a, 1);                                
                                };
                                b = 1;
                            }
                            end";
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    assert!(result.is_ok());
}

#[test]
fn test_expresion() {
    let source = "    program prueba2;  
                            var a,b : int;
                            var c,d : float;
                            main
                            {
                                a = b*c;
                                d = (a*c)/b + 10;
                            }
                            end";
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    assert!(result.is_ok());
}

#[test]
fn test_statement() {
    let source = "    program prueba3;  
                            main
                            {
                                print(\"Hola\");
                                i = 1;
                                while(i>1) do {
                                    FunctionCall();
                                    if(i<10) do {
                                        i = i*5;
                                    }
                                    else {
                                        i = i -5;
                                    };
                                };
                            }
                            end";
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    assert!(result.is_ok());
}

//Error

#[test]
fn test_variables() {
    let source = "    program 1prueba;  
                            var 1a,1b : int;
                            var 12c,43d : float;
                            void 1rafunc()[
                                {}
                            ];
                            main
                            {

                            }
                            end";
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    assert!(result.is_err());
}

#[test]
fn test_order() {
    let source = "    program prueba;  
                            void func()[
                                {}
                            ];
                            var a,b : int;
                            var c,d : float;

                            main
                            {

                            }
                            end";
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    assert!(result.is_err());
}

#[test]
fn test_reservedwords() {
    let source = "    progra prueba;  
                            
                            vr a,b : int;
                            vr c,d : float;
                            vod func()[
                                {}
                            ];
                            min
                            {

                            }
                            nd";
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    assert!(result.is_err());
}