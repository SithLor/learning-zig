package pixelpacking;
import java.util.*;

public class main {
    public static void main(String[] args) {
        int[] pixels = {20, 5, 20};
        int[] table = gen_color_table();
        packcolors(pixels, table);
        int[] e = pixels.clone();
        System.out.println(Arrays.toString(e));
    }
    public static int[] gen_color_table(){
        int[] table = new int[256];
        for (int i = 0; i < 256; i++) {
            table[i] = i;
        }
        return table;
    }
    public static void packcolors(int[] pixels, int[] table) {
        int[] packed = new int[pixels.length];
        for (int i = 0; i < pixels.length; i++) {
            packed[i] = table[pixels[i]];
        }
    }
}