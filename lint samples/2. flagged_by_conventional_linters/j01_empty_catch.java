public class EmptyCatch {
    public static void run() {
        try {
            int result = 10 / 0;
        } catch (ArithmeticException e) {
        }
    }
}