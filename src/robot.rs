use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct RobotArm {
    stream: TcpStream,
}

impl RobotArm {
    pub async fn new(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(address).await?;
        Ok(RobotArm { stream })
    }

    /// Moves the robot arm along a joint path.
    ///
    /// # Arguments
    ///
    /// * `j1` to `j6` - The joint angles in radians.
    /// * `a` - Optional acceleration.
    /// * `v` - Optional velocity.
    pub async fn movej(
        &mut self,
        j1: f32,
        j2: f32,
        j3: f32,
        j4: f32,
        j5: f32,
        j6: f32,
        a: Option<f32>,
        v: Option<f32>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let a_str = a.map_or(String::new(), |a| format!(", a={}", a));
        let v_str = v.map_or(String::new(), |v| format!(", v={}", v));
        let command = format!(
            "movej([{}, {}, {}, {}, {}, {}]{}{})\n",
            j1,
            j2,
            j3,
            j4,
            j5,
            j6,
            a_str,
            v_str
        );
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }
    /// Moves the robot arm along a linear path in its task space (cartesian space).
    ///
    /// The robot arm will move in a straight line between its current position
    /// and the target, with respect to the tool frame.
    ///
    /// # Arguments
    ///
    /// * `x`, `y`, `z` - Coordinates of the target position in meters.
    /// * `rx`, `ry`, `rz` - Rotation vector representation of the orientation of the tool.
    ///   Its magnitude is the angle in radians and its axis is aligned with the rotation axis.
    /// * `a` - Optional acceleration in `rad/s^2`. Determines how fast the robot reaches the target speed.
    /// * `v` - Optional target speed in `m/s` at which to move towards the target.
    pub async fn movel(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        rx: f32,
        ry: f32,
        rz: f32,
        a: Option<f32>,
        v: Option<f32>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let a_str = a.map_or(String::new(), |a| format!(", a={}", a));
        let v_str = v.map_or(String::new(), |v| format!(", v={}", v));
        let command = format!(
            "movel(p[{}, {}, {}, {}, {}, {}]{}{})\n",
            x,
            y,
            z,
            rx,
            ry,
            rz,
            a_str,
            v_str
        );
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }
    /// Bewegt den Roboterarm entlang eines Pfades im task space (kartesischer Raum),
    /// wobei die gegebene Pose entweder als linearer Punkt oder als Winkelziel behandelt wird.
    ///
    /// Der Unterschied zu `movel` besteht darin, dass bei `movep` der Weg im task space
    /// kontinuierlich geblendet wird, was zu einer glatteren Bewegung führt, besonders wenn
    /// mehrere movep-Befehle hintereinander ausgeführt werden.
    ///
    /// # Arguments
    ///
    /// * `x`, `y`, `z` - Koordinaten der Zielposition in Metern.
    /// * `rx`, `ry`, `rz` - Rotationsvektordarstellung der Ausrichtung des Werkzeugs.
    ///   Die Größe ist der Winkel in Radiant und die Achse ist mit der Rotationsachse ausgerichtet.
    /// * `a` - Optionale Beschleunigung in `rad/s^2`. Bestimmt, wie schnell der Roboter die Zielspeed erreicht.
    /// * `v` - Optionale Zielspeed in `m/s`, mit der sich der Roboter auf das Ziel zu bewegt.
    pub async fn movep(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        rx: f32,
        ry: f32,
        rz: f32,
        a: Option<f32>,
        v: Option<f32>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let a_str = a.map_or(String::new(), |a| format!(", a={}", a));
        let v_str = v.map_or(String::new(), |v| format!(", v={}", v));
        let command = format!(
            "movep(p[{}, {}, {}, {}, {}, {}]{}{})\n",
            x,
            y,
            z,
            rx,
            ry,
            rz,
            a_str,
            v_str
        );
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }
    /// Bewegt den Roboterarm entlang eines Kreisbogens, der durch drei Punkte definiert wird:
    /// den aktuellen Punkt des Roboters, einen Zwischenpunkt (`via`) und einen Endpunkt (`to`).
    ///
    /// # Arguments
    ///
    /// * `via_x`, `via_y`, `via_z` - Koordinaten des Zwischenpunktes in Metern.
    /// * `to_x`, `to_y`, `to_z` - Koordinaten des Endpunktes in Metern.
    /// * `via_rx`, `via_ry`, `via_rz` - Rotationsvektordarstellung der Ausrichtung des Werkzeugs am Zwischenpunkt.
    ///   Die Größe ist der Winkel in Radiant und die Achse ist mit der Rotationsachse ausgerichtet.
    /// * `to_rx`, `to_ry`, `to_rz` - Rotationsvektordarstellung der Ausrichtung des Werkzeugs am Endpunkt.
    /// * `a` - Optionale Beschleunigung in `rad/s^2`. Bestimmt, wie schnell der Roboter die Zielspeed erreicht.
    /// * `v` - Optionale Zielspeed in `m/s`, mit der sich der Roboter auf das Ziel zu bewegt.
    pub async fn movec(
        &mut self,
        via_x: f32,
        via_y: f32,
        via_z: f32,
        via_rx: f32,
        via_ry: f32,
        via_rz: f32,
        to_x: f32,
        to_y: f32,
        to_z: f32,
        to_rx: f32,
        to_ry: f32,
        to_rz: f32,
        a: Option<f32>,
        v: Option<f32>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let a_str = a.map_or(String::new(), |a| format!(", a={}", a));
        let v_str = v.map_or(String::new(), |v| format!(", v={}", v));
        let command = format!(
            "movec(p[{}, {}, {}, {}, {}, {}], p[{}, {}, {}, {}, {}, {}]{}{})\n",
            via_x,
            via_y,
            via_z,
            via_rx,
            via_ry,
            via_rz,
            to_x,
            to_y,
            to_z,
            to_rx,
            to_ry,
            to_rz,
            a_str,
            v_str
        );
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }
    /// Stoppt alle Gelenkbewegungen des Roboters.
    ///
    /// # Arguments
    ///
    /// * `a` - Optionale Abbremsrate in `rad/s^2`. Bestimmt, wie abrupt der Roboter stoppt.
    ///   Falls nicht angegeben, wird ein Standardwert verwendet.
    pub async fn stopj(&mut self, a: Option<f32>) -> Result<(), Box<dyn std::error::Error>> {
        let a_str = a.map_or(String::new(), |a| format!("({})", a));
        let command = format!("stopj{}\n", a_str);
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }
    /// Steuert die Geschwindigkeit des Roboterarms in Echtzeit für jede Achse.
    ///
    /// # Arguments
    ///
    /// * `qd` - Die Geschwindigkeit jeder Achse in `rad/s`. Die Geschwindigkeiten für Achsen q1 bis q6 werden separat übergeben.
    /// * `a` - Beschleunigungsrate in `rad/s^2`. Ein Wert zwischen 0.1 und 10 ist erlaubt.
    /// * `t` - Zeitdauer in Sekunden, für die dieser Geschwindigkeitsbefehl gelten soll.
    pub async fn speedj(
        &mut self,
        q1: f32,
        q2: f32,
        q3: f32,
        q4: f32,
        q5: f32,
        q6: f32,
        a: f32,
        t: f32
    ) -> Result<(), Box<dyn std::error::Error>> {
        if a < 0.1 || a > 10.0 {
            return Err(
                Box::new(
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid acceleration rate for speedj. Allowed values are between 0.1 and 10."
                    )
                )
            );
        }

        let command = format!(
            "speedj([{},{},{},{},{},{}], a={}, t={})\n",
            q1,
            q2,
            q3,
            q4,
            q5,
            q6,
            a,
            t
        );
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }
}