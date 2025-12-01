// gcc -c euclid.c -o euclid.o

#include <stdio.h>

// https://github.com/rohanthomas/Abstract-Algebra/blob/main/euclidean%20division%20algorithm.c
int rem_euclid(int a, int b) {
    int temp=a,r=1,count=0;

    // swap a,b if a is bigger than b  else don't
    if(a<b){
        a=b;
        b=temp;
    }

    /*Lemma: Let a=bq+r, then gcd(a,b)=gcd(b,r)
    We iteratively use this lemma until we reach a zero remainder.

    */

    while (r != 0) {
        r = a % b;
        a = b;
        b = r;
        count=count+1;
    }

    printf("The gcd is %d\n", a);

    // The number of steps taken is:
    return count-1;
}