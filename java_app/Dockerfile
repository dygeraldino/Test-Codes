FROM openjdk:25-jdk-slim-bookworm

WORKDIR /java_app

COPY . /java_app

RUN javac sc.java

CMD ["java", "sc"]