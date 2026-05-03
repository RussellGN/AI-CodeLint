public class ConcatLoop {
    public static String join(String[] words) {
        String result = "";
        for (String w : words) {
            result = result + w;
        }
        return result;
    }
}