class Solution {
    public int maxLength(int[] nums) {
        int n=nums.length,ans=0;
        for(int i=0;i<n;i++){
            int p=1,g=0,l=1;
            for(int j=i;j<n;j++){
                p*=nums[j];
                g=gcd(g,nums[j]);
                l=lcm(l,nums[j]);
                if(p==g*l) ans=Math.max(ans,j-i+1);
            }
        }
        return ans;
    }
    private int gcd(int a,int b){
        while(b!=0){
            int t=b;
            b=a%b;
            a=t;
        }
        return a;
    }
    private int lcm(int a,int b){
        return a/gcd(a,b)*b;
    }
}