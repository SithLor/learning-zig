namespace java {
    export abstract class Program {
        constructor(){
            let RETURN_CODE = this.main();
            switch(RETURN_CODE){
                case 0:
                    console.log('Success');
                    break;
                case 1:
                    console.log('Failed');
                    break;
            }
        }
        public abstract main():STATUS;
    }
    export type STATUS = 0|1
    
}

class Main extends java.Program{
    constructor(){
        super();
    }
    public main():java.STATUS{
        console.log('Hello World');
        let test:java.STATUS = 0;
        return test;
    }
}



const main = new Main();