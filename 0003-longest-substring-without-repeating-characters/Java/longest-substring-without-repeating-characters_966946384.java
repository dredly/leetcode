class Solution {
    public static int lengthOfLongestSubstring(String s) {
        if (s.length() == 0 || s.length() == 1) {
            return s.length();
        }
        var max = 0;
        ArrayList<Character> unique = new ArrayList<>();
        var charArray = s.toCharArray();
        for (char c: charArray) {
            var idx = unique.indexOf(c);
            if (idx == -1) {
                unique.add(c);
            } else {
                max = Math.max(max, unique.size());
                unique.add(c);
                for (int i = 0; i <= idx; i++) {
                    unique.remove(0);
                }
            }
        }
        return Math.max(max, unique.size());
    }
}