pipeline {
    agent none

    options {
        skipDefaultCheckout(true)
    }

    stages {
        stage('Check Commit Message') {
            agent any
            steps {
                checkout scm
                script {
                    def commitMessage = sh(script: 'git log -1 --pretty=%B', returnStdout: true).trim()
                    def skipBuild = (commitMessage =~ /\[skip ci\]|\[no ci\]/).find()

                    if (skipBuild) {
                        echo "Skipping build due to commit message containing [skip ci] or [no ci]"
                        currentBuild.result = 'NOT_BUILT'
                        error("Skipping build due to commit message")
                    }
                }
            }
        }

        stage('Build for Multiple Platforms') {
            parallel {
                stage('Linux x86_64') {
                    agent {
                        docker {
                            image 'rust:latest'
                            args '-v ${WORKSPACE}:/workspace'
                        }
                    }
                    steps {
                        sh '''
                            cd /workspace
                            rustup target add x86_64-unknown-linux-gnu
                            cargo build --release --target x86_64-unknown-linux-gnu
                        '''
                        stash includes: 'target/x86_64-unknown-linux-gnu/release/*.so', name: 'linux-x86_64'
                    }
                }

                stage('Linux ARM64') {
                    agent {
                        docker {
                            image 'rust:latest'
                            args '-v ${WORKSPACE}:/workspace -u root'
                        }
                    }
                    steps {
                        sh '''
                            cd /workspace
                            # Install ARM64 cross-compilation tools
                            apt-get update
                            apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

                            # Add ARM64 target
                            rustup target add aarch64-unknown-linux-gnu

                            # Create a .cargo/config.toml file to specify the linker
                            mkdir -p .cargo
                            cat > .cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

                            # Build with the proper target
                            cargo build --release --target aarch64-unknown-linux-gnu

                            chown -R 103:103 target/aarch64-unknown-linux-gnu
                        '''
                        stash includes: 'target/aarch64-unknown-linux-gnu/release/*.so', name: 'linux-arm64'
                    }
                }

                stage('Windows x86_64') {
                    agent {
                        docker {
                            image 'rust:latest'
                            args '-v ${WORKSPACE}:/workspace -u root'
                        }
                    }
                    steps {
                        sh '''
                            cd /workspace
                            rustup target add x86_64-pc-windows-gnu
                            apt-get update && apt-get install -y mingw-w64
                            cargo build --release --target x86_64-pc-windows-gnu
                            chown -R 103:103 target/x86_64-pc-windows-gnu
                        '''
                        stash includes: 'target/x86_64-pc-windows-gnu/release/*.dll', name: 'windows-x86_64'
                    }
                }
            }
        }

        stage('Collect Artifacts') {
            agent any
            steps {
                sh 'mkdir -p artifacts'

                unstash 'linux-x86_64'
                unstash 'linux-arm64'
                unstash 'windows-x86_64'

                sh '''
                    cp target/x86_64-unknown-linux-gnu/release/*.so artifacts/libcommandlimiter_x86_64_linux.so
                    cp target/aarch64-unknown-linux-gnu/release/*.so artifacts/libcommandlimiter_aarch64_linux.so
                    cp target/x86_64-pc-windows-gnu/release/*.dll artifacts/commandlimiter_x86_64_windows.dll
                '''

                archiveArtifacts artifacts: 'artifacts/*', fingerprint: true
            }
        }
    }
}
