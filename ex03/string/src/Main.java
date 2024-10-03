public class Main {
    static String reverseString(String str) {
        StringBuilder reversed = new StringBuilder(str);
        // pointer to start
        int start = 0;
        // pointer to end
        int end = reversed.length() - 1;

        while(start < end){
            char temp = reversed.charAt(start);
            // set start to end
            reversed.setCharAt(start, reversed.charAt(end));
            // set end to temp
            reversed.setCharAt(end, temp);
            start++;
            end--;
        }
        str = reversed.toString().strip();
        return str;
    }
    public static void main(String[] args) {
        String test = "Hello, world!";
        String reversed = reverseString(test);

        System.out.println(reversed);
    }
}