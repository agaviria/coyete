Coyete
======
Coyete [kahy-eh-tee] it's named after `cohete` meaning `rocket` in
spanish.  The name is distinct only because it rolls off the tongue better.

This is an experiment repository to understand [Rocket Framework](https://rocket.rs) and other plugins.

During build, you need the following env var:

    DATABASE_URL=database path for diesel to function properly.
    COYETE_BUILDTIME_PEPPER=some 32-byte random value encoded with Base64 (usually 44 ASCII characters) used for password hashes.

Optional .env variable to run the application in production mode:

    RUN_MODE=Production
