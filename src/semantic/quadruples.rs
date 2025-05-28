#![allow(warnings)]
use std::collections::HashMap;

use crate::semantic::cube::SemanticCube;
use crate::ast::{DataType, Operator, Value, Expression, Context};

use crate::utils::stack::Stack;
use crate::memory::variables::{VariableValueDirectory };


#[derive(Debug)]
enum PolacItem {
    Operator(Operator), 
    Address(usize),     
}
#[derive(Debug)]
pub struct Quadruple{
    pub iOperator: Operator,
    //Guarda la direccion de memoria de los argumentos
    pub oArg1: Option<usize>, 
    pub oArg2: Option<usize>,
    pub oResult: Option<usize>
}

impl Quadruple{
    pub fn new(_iOperator: Operator, _oArg1: Option<usize>, _oArg2: Option<usize>, _oResult: Option<usize>) -> Self {
        Self { 
            iOperator: _iOperator, 
            oArg1: _oArg1, 
            oArg2: _oArg2, 
            oResult: _oResult
        }
    }     
}

#[derive(Debug)]
pub struct QuadrupleList {
    //Tabla de Variables parecida a la directory para las funciones, pero aqui solo guarda temporales
    //Usa la misma tabla de variables de memoria, y tambien se usa para guardar las constantes
    oVariableTempTable: HashMap<String, usize>, // El usize minimo es el que esta en Variables: 4000

    //Stack de operadores
    oOperatorStack: Stack<Operator>,

    //Stack de Variables y Constantes, solo guarda la memoria
    oOperandStack: Stack<usize>,

    //Stack de Variables y Constantes, solo guarda la memoria
    oJumpsStack: Stack<usize>,

    //Vector de cuadruplos, aqui se registraran todas las acciones
    oQuadruples: Vec<Quadruple>,

    pub oContext: Context
}

impl QuadrupleList {

    pub fn new() -> Self {
        Self {
            oVariableTempTable: HashMap::new(),
            oOperatorStack: Stack::new(),
            oOperandStack: Stack::new(),    
            oJumpsStack: Stack::new(),
            oQuadruples: Vec::new(),
            oContext: Context::Global // La lista de cuadruplos siempre iniciara en Global
        }
    }

    pub fn getSize(&self) -> usize{
        return self.oQuadruples.len();
    }

    pub fn getQuadruple(&self, uIndex: usize) -> &Quadruple{
        return &self.oQuadruples[uIndex];
    }

    pub fn InsertOperand(&mut self, _uVar: usize){
        self.oOperandStack.push(_uVar);
    }

    pub fn InsertOperator(&mut self, _oOperator: Operator){
        self.oOperatorStack.push(_oOperator);
    }

    pub fn InsertJump(&mut self, iQuadrupleIndex: usize){
        
        self.oJumpsStack.push(iQuadrupleIndex);
    }

    pub fn DeleteOperator(&mut self){
        self.oOperatorStack.pop();
    }

    

    pub fn CheckSemanticCube(&mut self,uRighOperand: usize, uLeftOperand: usize, _oOperator: Operator) -> Option<DataType>{
        let oSemanticCube = SemanticCube::new();

        let mut oRightOperandType: DataType = DataType::GetTypeFromContext(uRighOperand);
        let mut oLeftOperandType: DataType = DataType::GetTypeFromContext(uLeftOperand);
        
        oSemanticCube.result_type(oLeftOperandType, oRightOperandType, _oOperator)

    }

    pub fn GenerateQuadrupleGoto(&mut self, uJump: Option<usize>){               
        
        let oOperator = self.oOperatorStack.pop().unwrap();      
        

        let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                        None, 
                                                        None,
                                                            uJump
                                                    );
        self.oQuadruples.push(oQuadrupleTemp);    
    }
    
    // Genera Cuadruplo para todos los operadores
    // Regresa Bool: True: Si funciono | False : No Funciono
     pub fn GenerateQuadruple(&mut self, oVariableValueDirectory: &mut VariableValueDirectory) -> bool{
        if let Some(operator_ref) = self.oOperatorStack.peek() {
            match operator_ref {
                Operator::Add | Operator::Sub |
                Operator::Mul | Operator::Div | 
                Operator::GreaterThan | Operator::LessThan |
                Operator::NotEqual  => {
                   
                    let uRightOperand = self.oOperandStack.pop().unwrap();
                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let uLeftOperand = self.oOperandStack.pop().unwrap();        

                    let oResultType = self.CheckSemanticCube(uRightOperand, uLeftOperand, oOperator);
                    
                    if let Some(oType) = oResultType {
                        // Aqui se insertara el valor del resultado de las expresiones
                        match oVariableValueDirectory.generateVariable(Context::Temp, // Como aqui se generan los resultados de operaciones, se guarda en el contexto temporal
                                                                    oType.DefaultValue(), 
                                                                    oType) { 
                        Ok(result_address) => {
                            let quad = Quadruple::new(oOperator, 
                                                            Some(uLeftOperand),
                                                            Some(uRightOperand), 
                                                            Some(result_address));
                            self.oQuadruples.push(quad);

                            self.oOperandStack.push(result_address);
                            return true;
                        }
                        Err(_) => {return false;},
                    }
                        
                    } else {
                        panic!("Type mismatch: {:?} {:?} {:?}", uLeftOperand, oOperator, uRightOperand);
                        return false;
                    }
                }       
                Operator::Assign => {
                    let oVariable = self.oOperandStack.pop();

                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let oResult = self.oOperandStack.pop();//self.oQuadruples.last().unwrap().oResult;
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    oResult, // Valor que sera asignado
                                                                    None,
                                                                     oVariable // Variable donde se asignara el valor
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);
                }   
                Operator::Print => {
                    //let oVariable = self.oOperandStack.pop();

                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let oResult = self.oOperandStack.pop();//self.oQuadruples.last().unwrap().oResult;
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    None, // Valor que sera asignado
                                                                    None,
                                                                     oResult // Variable donde se asignara el valor
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);
                }
                  
                _ => { // Para cualquier otro caso
                    return false;
                }
            }
        }
        false 
    }

    // Genera Cuadruplo para Condicional If
    pub fn GenerateQuadrupleConditional(&mut self) -> bool{
        if let Some(operator_ref) = self.oOperatorStack.peek() {
            match operator_ref {                
                Operator::GotoF => {
                    
                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let oResult = self.oOperandStack.pop();//self.oQuadruples.last().unwrap().oResult;
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    oResult, 
                                                                    None,
                                                                     None
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);
                    // Obtener el indice del quadruplo que ocupa saber el salto siguiente
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); 
                    self.InsertJump(iQuadrupleIndex.unwrap());                                         
                } ,
                Operator::Goto => {
                    //Genera Cuadruplo GOTO Vacio
                    self.GenerateQuadrupleGoto(None);  
                    self.FinishJump();
                    // Obtener el indice del quadruplo que ocupa saber el salto siguiente
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); 
                    self.oJumpsStack.push(iQuadrupleIndex.unwrap());   
                }
   
                _ => { // Para cualquier otro caso
                    return false;
                }
            }
        }
        false 
    }

    // Genera Cuadruplo de Ciclos como While 
    pub fn GenerateQuadrupleCycle(&mut self) -> bool{
        if let Some(operator_ref) = self.oOperatorStack.peek() {
            match operator_ref {                
                Operator::GotoF => {
                    
                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let oResult = self.oOperandStack.pop();//self.oQuadruples.last().unwrap().oResult;
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    oResult, 
                                                                    None,
                                                                     None
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);
                    // Obtener el indice del quadruplo que ocupa saber el salto siguiente
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); 
                    self.InsertJump(iQuadrupleIndex.unwrap());                                
                } ,
                Operator::Goto => {
                    let iEnd =self.oJumpsStack.pop().unwrap();
                    let iReturn =self.oJumpsStack.pop();          

                    
                    self.GenerateQuadrupleGoto(iReturn);                  

                    // Rellena el GOTO generado para regrese al inicio del ciclo
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); // Cuadruplo GOTO Generado
                    self.InsertJump(iQuadrupleIndex.unwrap());  // Insertar a jump el indice actual 

                    self.FillGotoQuadruple(iEnd); // Ingresa el el inicio del goto                    

                    // Se estaba quedando un salto guardado checar si no causa error:
                    self.oJumpsStack.pop();
            }       
   
                _ => { // Para cualquier otro caso
                    return false;
                }
            }
        }
        false 
    }

    //Para los Goto ya generados, se ejecuta cuando ya se encontro el final del metodo
    pub fn FillGotoQuadruple(&mut self, iQuadrupleIndex: usize) -> bool{
        //Guardar en el Goto el indice a donde saltara
        self.oQuadruples[iQuadrupleIndex].oResult = Some(self.oQuadruples.len());
        return true;
    }

    pub fn FinishJump(&mut self) -> bool{
        let iEnd = self.oJumpsStack.pop().unwrap();
        return self.FillGotoQuadruple(iEnd);
    }


    pub fn print_table(&self) {

        println!("Saltos: {:?}", self.oJumpsStack);
        println!("Operandos: {:?}", self.oOperandStack);
        println!("Operadores: {:?}", self.oOperatorStack);

        println!("{:<5} | {:<12} | {:<10} | {:<10} | {:<10}", 
                 "Idx", "Operator", "Arg1", "Arg2", "Result");
        println!("{}", "-".repeat(60));

        for (i, q) in self.oQuadruples.iter().enumerate() {
            println!("{:<5} | {:<12?} | {:<10} | {:<10} | {:<10}", 
                i, 
                q.iOperator,
                match q.oArg1 {
                    Some(val) => val.to_string(),
                    None => "_".to_string(),
                },
                match q.oArg2 {
                    Some(val) => val.to_string(),
                    None => "_".to_string(),
                },
                match q.oResult {
                    Some(val) => val.to_string(),
                    None => "_".to_string(),
                }
            );
        }
    }


}
