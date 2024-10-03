#include <stdio.h>
#include <stdlib.h>
#include <string.h>


void reverseString(char *str) {
    char *start = str;                // Pointer to the start of the string
    char *end = str + strlen(str) - 1; // Pointer to the end of the string

    while (start < end) {
        // Swap the characters
        char temp = *start;
        *start = *end;
        *end = temp;

        // Move the pointers towards each other
        start++;
        end--;
    }
}

int main() {
    int size = 30;

    // Allocate buffer for the string
    char *stringBuffer = (char *)malloc(size * sizeof(char));

    if (stringBuffer == NULL) {
        printf("Couldn't allocate memory.\n");
        exit(1);
    }

    memset(stringBuffer, '\0', sizeof(char) * size);

    strcpy(stringBuffer, "Hello, world!");

    printf("Original string: %s\n", stringBuffer);

    // Reverse the string in-place
    reverseString(stringBuffer);

    printf("Reversed string: %s\n", stringBuffer);

    free(stringBuffer);

    return 0;
}
