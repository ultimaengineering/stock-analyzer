FROM ubuntu:21.04
RUN apt-get update && apt-get upgrade -y
RUN mkdir /app
RUN ls /workspace
ADD stock_analyzer /app/stock_analyzer
CMD /app/stock_analyzer
