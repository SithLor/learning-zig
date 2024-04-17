
public class StarsAndStripes {
    public StarsAndStripes() {
        System.out.println("StarsAndStripes");
        printTwoBlankLines();
    }

    public void printTwentyStars() {
        System.out.println("********************");
    }

    public void printTwentyDashes() {
        System.out.println("--------------------");
    }

    public void printTwoBlankLines() {
        System.out.println("\n");
    }

    public void printASmallBox() {
        printTwentyDashes();
        printTwentyStars();
        printTwentyDashes();
    }

    public void printABigBox() {
        printASmallBox();
        printASmallBox();
    }

    public static void main(String[] args) {
        StarsAndStripes starsAndStripes = new StarsAndStripes();
        starsAndStripes.printABigBox();
    }
}