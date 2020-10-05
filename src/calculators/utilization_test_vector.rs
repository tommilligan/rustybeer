use crate::calculators::ibu::test::TestVector;
pub fn get_vector() -> TestVector
{
    TestVector{
        og : vec![1.030, 1.040, 1.050, 1.060, 1.070, 1.080, 1.090, 1.100, 1.110, 1.120, 1.130],
        boiling_time: vec![ 0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60, 70, 80, 90, 120],
        utilization: vec![
            vec![0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000],
            vec![0.034, 0.031, 0.029, 0.026, 0.024, 0.022, 0.020, 0.018, 0.017, 0.015, 0.014],
            vec![0.065, 0.059, 0.054, 0.049, 0.045, 0.041, 0.038, 0.035, 0.032, 0.029, 0.026],
            vec![0.092, 0.084, 0.077, 0.070, 0.064, 0.059, 0.054, 0.049, 0.045, 0.041, 0.037],
            vec![0.116, 0.106, 0.097, 0.088, 0.081, 0.074, 0.068, 0.062, 0.056, 0.052, 0.047],
            vec![0.137, 0.125, 0.114, 0.105, 0.096, 0.087, 0.080, 0.073, 0.067, 0.061, 0.056],
            vec![0.156, 0.142, 0.130, 0.119, 0.109, 0.099, 0.091, 0.083, 0.076, 0.069, 0.063],
            vec![0.173, 0.158, 0.144, 0.132, 0.120, 0.110, 0.101, 0.092, 0.084, 0.077, 0.070],
            vec![0.187, 0.171, 0.157, 0.143, 0.131, 0.120, 0.109, 0.100, 0.091, 0.083, 0.076],
            vec![0.201, 0.183, 0.168, 0.153, 0.140, 0.128, 0.117, 0.107, 0.098, 0.089, 0.082],
            vec![0.212, 0.194, 0.177, 0.162, 0.148, 0.135, 0.124, 0.113, 0.103, 0.094, 0.086],
            vec![0.223, 0.203, 0.186, 0.170, 0.155, 0.142, 0.130, 0.119, 0.108, 0.099, 0.091],
            vec![0.232, 0.212, 0.194, 0.177, 0.162, 0.148, 0.135, 0.124, 0.113, 0.103, 0.094],
            vec![0.240, 0.219, 0.200, 0.183, 0.167, 0.153, 0.140, 0.128, 0.117, 0.107, 0.098],
            vec![0.247, 0.226, 0.206, 0.189, 0.172, 0.158, 0.144, 0.132, 0.120, 0.110, 0.101],
            vec![0.253, 0.232, 0.212, 0.194, 0.177, 0.162, 0.148, 0.135, 0.123, 0.113, 0.103],
            vec![0.259, 0.237, 0.216, 0.198, 0.181, 0.165, 0.151, 0.138, 0.126, 0.115, 0.105],
            vec![0.264, 0.241, 0.221, 0.202, 0.184, 0.169, 0.154, 0.141, 0.129, 0.118, 0.108],
            vec![0.269, 0.246, 0.224, 0.205, 0.188, 0.171, 0.157, 0.143, 0.131, 0.120, 0.109],
            vec![0.273, 0.249, 0.228, 0.208, 0.190, 0.174, 0.159, 0.145, 0.133, 0.121, 0.111],
            vec![0.276, 0.252, 0.231, 0.211, 0.193, 0.176, 0.161, 0.147, 0.135, 0.123, 0.112],
            vec![0.285, 0.261, 0.238, 0.218, 0.199, 0.182, 0.166, 0.152, 0.139, 0.127, 0.116],
            vec![0.291, 0.266, 0.243, 0.222, 0.203, 0.186, 0.170, 0.155, 0.142, 0.130, 0.119],
            vec![0.295, 0.270, 0.247, 0.226, 0.206, 0.188, 0.172, 0.157, 0.144, 0.132, 0.120],
            vec![0.301, 0.275, 0.252, 0.230, 0.210, 0.192, 0.176, 0.161, 0.147, 0.134, 0.123],
        ]
    }
}