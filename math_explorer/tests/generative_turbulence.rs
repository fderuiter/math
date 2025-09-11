// tests/generative_turbulence.rs

use math_explorer::applied::generative_turbulence::models::adv_no::AdvNO;
use math_explorer::applied::generative_turbulence::losses::{generator_loss, discriminator_loss, perceptual_loss, Vgg19Features};
use tch::{nn::{self, OptimizerConfig, Module}, Device, Tensor, Kind};

#[test]
fn test_adv_no_training_step() {
    let device = Device::Cpu;
    let vs_gen = nn::VarStore::new(device);
    let vs_disc = nn::VarStore::new(device);
    let vs_vgg = nn::VarStore::new(device);

    let mut opt_gen = nn::Adam::default().build(&vs_gen, 1e-4).unwrap();
    let mut opt_disc = nn::Adam::default().build(&vs_disc, 1e-4).unwrap();

    // Model parameters for 2D super-resolution (LRLF -> HRHF)
    // LRLF input: [u(t), u(t+4t)] -> 2 channels
    // HRHF output: [u(t), u(t+t), u(t+2t), u(t+3t), u(t+4t)] -> 5 channels
    let c_in = 2;
    let c_out = 5;
    let c_init = 8; // Very small for fast testing

    let adv_no = AdvNO::new(&vs_gen, &vs_disc, c_in, c_out, c_init);

    // Perceptual loss model
    // In a real scenario, we would load pre-trained weights into vs_vgg
    let vgg_features = Vgg19Features::new(&vs_vgg.root(), device);

    // Dummy data
    let batch_size = 2;
    let lrlf_res = 16;
    let hrhf_res = 128; // 16 * 8
    let lrlf_data = Tensor::rand(&[batch_size, c_in, lrlf_res, lrlf_res], (Kind::Float, device));
    let hrhf_data = Tensor::rand(&[batch_size, c_out, hrhf_res, hrhf_res], (Kind::Float, device));
    let time_steps = Tensor::rand(&[batch_size], (Kind::Float, device));

    // --- Generator Forward Pass ---
    let fake_hrhf = adv_no.generator.forward_with_time(&lrlf_data, Some(&time_steps));
    assert_eq!(fake_hrhf.size(), hrhf_data.size());
    println!("Test passed: Generator output shape is correct.");

    // --- Discriminator Update ---
    let fake_hrhf_detached = fake_hrhf.detach();
    let fake_pred = adv_no.discriminator.forward(&fake_hrhf_detached);
    let real_pred = adv_no.discriminator.forward(&hrhf_data);
    println!("Test passed: Discriminator forward pass is ok.");

    let loss_d = discriminator_loss(&real_pred, &fake_pred);
    opt_disc.zero_grad();
    loss_d.backward();
    opt_disc.step();
    println!("Discriminator loss: {}", loss_d.double_value(&[]));

    // --- Generator Update ---
    let fake_pred_for_gen_loss = adv_no.discriminator.forward(&fake_hrhf);
    // Re-run real pass for generator loss, but detach it as we don't train the discriminator here.
    let real_pred_for_gen_loss = adv_no.discriminator.forward(&hrhf_data).detach();
    let loss_g_adv = generator_loss(&real_pred_for_gen_loss, &fake_pred_for_gen_loss);
    let loss_perceptual = perceptual_loss(&fake_hrhf, &hrhf_data, &vgg_features);
    let loss_l1 = fake_hrhf.l1_loss(&hrhf_data, tch::Reduction::Mean);

    let total_loss_g = loss_g_adv + loss_perceptual * 10.0 + loss_l1 * 10.0;
    opt_gen.zero_grad();
    total_loss_g.backward();
    opt_gen.step();
    println!("Generator total loss: {}", total_loss_g.double_value(&[]));

    println!("Test passed: Full training step ran without errors.");
}
