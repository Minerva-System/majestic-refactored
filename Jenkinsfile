podTemplate(containers: [
	containerTemplate(
		name: 'rust-builder',
		image: 'rust:1.68.1-alpine'
	)
]) {
	node(POD_LABEL) {
		stage('Rust project build') {
			container('rust-builder') {
				stage('Build') {
					sh 'cargo build --release'
				}
			}
		}
	}
}
