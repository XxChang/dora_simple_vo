# dora_simple_vo
This experimental repo shows how to run [openvslam](https://github.com/stella-cv/stella_vslam) on [Dora](https://github.com/dora-rs/dora).

## How to run

Please install [Dora](https://dora.carsmos.ai/docs/guides/Installation/installing) and [Openvslam](https://stella-cv.readthedocs.io/en/latest/installation.html) firstly.

And then, download an ORB vocabulary from GitHub
```
curl -sL "https://github.com/stella-cv/FBoW_orb_vocab/raw/main/orb_vocab.fbow" -o orb_vocab.fbow
```

To run this example, we also should download the [SRRG-Formated](https://github.com/NamDinhRobotics/proSLAM) dataset (please use KITTI dataset). After downloading the dataset, please modify [dataflow.yml](dataflow.yml).

Finally!
```
cargo run
```
