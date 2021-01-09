## Build

```
git clone https://github.com/DK99/prusaslicer_autocxx.git
cd prusaslicer_autocxx
docker build -t prusaslicer_autocxx .
docker run -v `pwd`:/data prusaslicer_autocxx:latest
```