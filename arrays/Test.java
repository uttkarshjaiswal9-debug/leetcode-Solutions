public class Test {
    public static void main(String[] args) {
        ValidParenhness vp = new ValidParenhness();
        System.out.println(vp.isValid("()"));
        System.out.println(vp.isValid("()[]{}"));
        System.out.println(vp.isValid("(]"));
        System.out.println(vp.isValid("([)]"));
    }
}
