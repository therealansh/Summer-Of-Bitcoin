#include<bits/stdc++.h>
using namespace std;


int dp[1005][1005];

int knapSack(int w, int wt[], int val[], int n){ 
    memset(dp,0,sizeof(dp));
    for(int i=1;i<=w;i++){
        for(int j=1;j<=n;j++){
            if(wt[j-1]<=i)
            dp[i][j]=max(dp[i-wt[j-1]][j-1]+val[j-1],dp[i][j-1]);
            else
            dp[i][j]=dp[i][j-1];
        }
    }
    return dp[w][n];
}



int main()
 {
    int t;
    cin>>t;
    while(t--)
    {
        int n, w;
        cin>>n>>w;
        
        int val[n];
        int wt[n];
        
        for(int i=0;i<n;i++)
            cin>>val[i];
        
        for(int i=0;i<n;i++)
            cin>>wt[i];
        
        cout<<knapSack(w, wt, val, n)<<endl;
        
    }
	return 0;
} 
