pipeline {
  agent {
    kubernetes {
      yamlFile 'KubernetesBuilder.yaml'
    }
  }
  stages {
    stage('Build') {
      steps {
        checkout scm
        container('rust') {
          sh 'cargo build'
        }
      }
    }
    stage('Test') {
      steps {
        checkout scm
        container('rust') {
          withCredentials([string(credentialsId: 'alpaca_secret_key', variable: 'alpaca_secret_key')]) {
            withCredentials([string(credentialsId: 'alpaca_access_key', variable: 'alpaca_access_key')]) {
              withCredentials([string(credentialsId: 'coveralls_stock_analyzer', variable: 'coveralls_stock_analyzer')]) {
                sh 'cargo test'
                sh 'cargo tarpaulin --coveralls ${coveralls_stock_analyzer}'
              }
            }
          }
        }
      }
    }
    stage('Copy Artifacts') {
      steps {
        container('rust') {
          sh 'cp target/release/stock-analyzer /workspace/opt/app/shared/stock-analyzer'
          sh 'cp Dockerfile /workspace/opt/app/shared/Dockerfile'
        }
      }
    }
    stage('Release') {
      steps {
        container('kaniko') {
          sh 'cp /workspace/opt/app/shared/stock-analyzer  /workspace'
          sh 'cp /workspace/opt/app/shared/Dockerfile /workspace'
          sh 'ulimit -n 10000'
          sh '/kaniko/executor -f Dockerfile --destination=docker.ultimaengineering.io/stock-analyzer:${BRANCH_NAME}-${BUILD_NUMBER}'
        }
      }
    }
  }
}