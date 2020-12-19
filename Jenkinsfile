podTemplate(yaml: """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: builder
spec:
  containers:
  - name: rust
    image: rust:1.48-stretch
    command:
    - cat
    tty: true
    volumeMounts:
    - mountPath: '/workspace/opt/app/shared/'
      name: sharedvolume
  - name: kaniko
    workingDir: /tmp/jenkins
    image: gcr.io/kaniko-project/executor:debug-v0.14.0
    imagePullPolicy: Always
    capabilities:
      add: ["IPC_LOCK"]
    command:
    - /busybox/cat
    tty: true
    volumeMounts:
    - mountPath: '/workspace/opt/app/shared/'
      name: sharedvolume
  volumes:
      - name: sharedvolume
        persistentVolumeClaim:
          claimName: sharedvolume"""
  ) {
  node(POD_LABEL) {
        stage('Build and test') {
          checkout scm
          container('rust') {
            sh 'cargo test'
            sh 'cargo build --release'
            sh 'cp -r target/release/stock-analyzer /workspace/opt/app/shared/app'
          }
        }
        stage('Build with Kaniko') {
             environment {
              PATH = "/busybox:/kaniko:$PATH"
             }
              container(name: 'kaniko', shell: '/busybox/sh') {
               sh 'cp -r /workspace/opt/app/shared/* /workspace/'
               sh 'pwd'
               sh 'ulimit -n 10000'
               sh '/kaniko/executor -f Dockerfile --destination=docker.ultimaengineering.io/stock-analyzer:latest'
              }
             }
}}