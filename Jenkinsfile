pipeline {
    agent any
    stages {
        stage('Git Checkout') {
            steps {
                script {
                    checkout scm
                }
            }
        }

        stage('Test code') {
            steps{
                script {
                    def tests = sh(script: "cargo test", returnStdout: true ).tokenize("\n").findAll{line -> (line =~ /error$|result:/)}
                    echo "Tests: ${tests}"
                }
            }
        }   
    }
}
