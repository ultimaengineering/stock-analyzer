FROM ubuntu:21.04
RUN apt-get update && apt-get upgrade -y && apt-get install sh -y
RUN mkdir /app
RUN ls /workspace
ADD stock-analyzer /app/stock-analyzer
CMD /app/stock-analyzer
