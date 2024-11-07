use eframe::{egui, App, NativeOptions};
use egui::{Color32, Stroke, Vec2};
use std::f32::consts::TAU;

struct SimulatorApp {
    // Parametri della simulazione
    radius: f32,
    angular_velocity: f32, // radianti al secondo
    velocity: f32,
    // Stato della simulazione
    angle: f32, // angolo corrente in radianti
    last_update: std::time::Instant,
}

impl Default for SimulatorApp {
    fn default() -> Self {
        Self {
            radius: 100.0,
            angular_velocity: TAU / 4.0, // 90 gradi al secondo
            velocity: 0.0,
            angle: 0.0,
            last_update: std::time::Instant::now(),
        }
    }
}

impl App for SimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Limita il framerate a 60 FPS
        ctx.request_repaint_after(std::time::Duration::from_secs_f32(1.0 / 60.0));

        let now = std::time::Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Aggiorna lo stato della simulazione
        self.angle += self.angular_velocity * delta_time;
        self.angle %= TAU;
        self.velocity = self.angular_velocity * self.radius;

        // Pannello laterale per i controlli
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Controlli");

            // Slider per il raggio
            ui.add(
                egui::Slider::new(&mut self.radius, 10.0..=200.0)
                    .text("Raggio")
                    .suffix(" unità"),
            );

            // Slider per la velocità angolare
            ui.add(
                egui::Slider::new(&mut self.angular_velocity, -2.0 * TAU..=2.0 * TAU)
                    .text("Velocità Angolare")
                    .suffix(" rad/s"),
            );

            // Mostra la velocità (calcolata)
            ui.label(format!("Velocità: {:.2} unità/s", self.velocity));

            // Calcola e mostra l'accelerazione centripeta
            let centripetal_acceleration = self.angular_velocity.powi(2) * self.radius;
            ui.label(format!(
                "Accelerazione Centripeta: {:.2} unità/s²",
                centripetal_acceleration
            ));

            // Calcola e mostra la frequenza
            let frequency = self.angular_velocity / TAU;
            ui.label(format!("Frequenza: {:.2} Hz", frequency.abs()));
        });

        // Pannello centrale per la simulazione
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simulazione del Moto Circolare Uniforme");

            // Area di visualizzazione della simulazione
            let (response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());

            let center = response.rect.center();

            // Disegna il percorso circolare
            painter.circle_stroke(
                center,
                self.radius,
                Stroke::new(2.0, Color32::LIGHT_GRAY),
            );

            // Calcola la posizione dell'oggetto in movimento
            let x = self.radius * self.angle.cos();
            let y = self.radius * self.angle.sin();

            let pos = center + Vec2::new(x, y);

            // Disegna l'oggetto in movimento
            painter.circle_filled(pos, 8.0, Color32::RED);

            // Calcola il vettore velocità
            let vx = -self.velocity * self.angle.sin();
            let vy = self.velocity * self.angle.cos();

            let velocity_vector = Vec2::new(vx, vy);

            // Verifica che il vettore velocità non sia nullo
            if velocity_vector.length_sq() > 0.0 {
                // Normalizza il vettore velocità
                let velocity_direction = velocity_vector.normalized();

                // Definisci la lunghezza della freccia per scopi di visualizzazione
                let arrow_length = 50.0;

                // Calcola il punto finale della freccia
                let arrow_end = pos + velocity_direction * arrow_length;

                // Disegna la linea principale della freccia
                painter.line_segment(
                    [pos, arrow_end],
                    Stroke::new(2.0, Color32::BLUE),
                );

                // Calcola il vettore perpendicolare
                let perp = Vec2::new(-velocity_direction.y, velocity_direction.x);

                // Definisci la dimensione delle ali della freccia
                let arrowhead_size = 10.0;

                // Calcola i punti delle ali della freccia
                let left = arrow_end - velocity_direction * arrowhead_size + perp * arrowhead_size * 0.5;
                let right = arrow_end - velocity_direction * arrowhead_size - perp * arrowhead_size * 0.5;

                // Disegna le ali della freccia
                painter.line_segment(
                    [arrow_end, left],
                    Stroke::new(2.0, Color32::BLUE),
                );
                painter.line_segment(
                    [arrow_end, right],
                    Stroke::new(2.0, Color32::BLUE),
                );
            }
        });
    }
}

fn main() {
    let app = SimulatorApp::default();
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Simulatore di Moto Circolare Uniforme",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    );
}
