FROM gcc:latest AS build

WORKDIR /src

COPY <<EOF /src/main.c
#include <stdio.h>

int main() {
    printf("Hello world!");
    return 0;
}
EOF

RUN gcc -static main.c -o app 
#use static because scratch doesnt have libraries

FROM scratch
COPY --from=build /src/app /app
CMD ["/app"]