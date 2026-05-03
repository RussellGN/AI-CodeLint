public class IndexCheck {
    public static int find(int[] arr, int target) {
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] == target) return i;
        }
        return -1;
    }
}