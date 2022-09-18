pub mod is_04 {
    pub mod v1_0_x {
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!("schemas/is_04/v1_0_x.json");
    }

    pub mod v1_1_x {
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!("schemas/is_04/v1_1_x.json");
    }

    pub mod v1_2_x {
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!("schemas/is_04/v1_2_x.json");
    }

    pub mod v1_3_x {
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!("schemas/is_04/v1_3_x.json");
    }
}

pub mod is_05 {
    pub mod v1_0_x {
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!("schemas/is_05/v1_0_x.json");
    }

    pub mod v1_1_x {
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!("schemas/is_05/v1_1_x.json");
    }
}
