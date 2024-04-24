public class Main {
    // MAIN CODE HERE
    class CashRegister {
        private int money;
        public CashRegister(){
            this.money = 0;
        }
        public void add(int amount){
            this.money += amount;
        }
        public int getMoney(){
            return this.money;
        }
        public int giveChange(int payment,int amount){
            return payment -= amount;
        }
        public void reset(){
            this.money = 0;
        }
    }
    
    class Bank {
        private int money;
        private int id;
        private int transaction;
        public Bank(){
            this.money = 0;
            this.id = 0;
            this.transaction = 0;
        }
        public void add(int amount){
            this.transaction += 1;

            if (amount >= 500){
                System.out.println("NO")
            }
            this.money += amount;
        }
        public void withdraw(int amount){
            this.transaction += 1;
            //check if amount is greater than money
            if (amount > this.money){
                System.out.println("NO")
            } else {
                this.money -= amount;
            }
            this.money -= amount;
        }
        public int get_id(){
            return this.id;
        }
        public int getMoney(){
            this.transaction += 1;
            return this.money;

        }
        public int tranfer(){
            this.transaction += 1;
            int amount = this.money;
            this.money = 0;
            return amount
        }
    }

    public void code() {
        CashRegister register = new CashRegister();
        System.out.println(register.getMoney());
        register.add(3);
        System.out.println(register.getMoney());
        register.add(2);
        System.out.println(register.getMoney());
        register.reset();
        System.out.println(register.getMoney());
    }


    // this code create make main class object and call code method to get round static
    public static void main(String[] args) {
        Main main = new Main();
        main.code();
    }
    

}