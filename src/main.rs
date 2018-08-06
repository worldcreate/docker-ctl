use docker_ctl::DockerCtl;

mod docker_ctl;

fn main() {
    match DockerCtl::new() {
        Ok(docker_ctl) => {
           docker_ctl.run();
        },
        Err(error) => {
            panic!(error);
        }
    };
}
