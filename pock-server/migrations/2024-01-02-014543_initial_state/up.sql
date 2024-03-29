CREATE SEQUENCE "user_id_seq";
CREATE TABLE "user" ("id" INT DEFAULT nextval('"user_id_seq"') NOT NULL, "name" varchar NOT NULL, CONSTRAINT "UQ_065d4d8f3b5adb4a08841eae3c8" UNIQUE ("name"), CONSTRAINT "PK_cace4a159ff9f2512dd42373760" PRIMARY KEY ("id"));
CREATE SEQUENCE "trip_id_seq";
CREATE TABLE "trip" ("id" INT DEFAULT nextval('"trip_id_seq"') NOT NULL, "name" varchar NOT NULL, "description" varchar NOT NULL, CONSTRAINT "UQ_fa18052e4307560de2ef1be2951" UNIQUE ("name"), CONSTRAINT "PK_714c23d558208081dbccb9d9268" PRIMARY KEY ("id"));
CREATE SEQUENCE "transaction_id_seq";
CREATE TABLE "transaction" ("id" INT DEFAULT nextval('"transaction_id_seq"') NOT NULL, "name" varchar NOT NULL, "description" varchar NOT NULL, "value" float8 NOT NULL, "tripId" int8, "payerId" int8, CONSTRAINT "PK_89eadb93a89810556e1cbcd6ab9" PRIMARY KEY ("id"));
CREATE INDEX "IDX_8639071696032d9f59b2623bd2" ON "transaction" ("tripId") ;
CREATE INDEX "IDX_e32e0d2862e47419a5bb737078" ON "transaction" ("payerId") ;
CREATE TABLE "transaction_participants_user" ("transactionId" int8 NOT NULL, "userId" int8 NOT NULL, CONSTRAINT "PK_5740e1f10c35c34d6b84ce3c243" PRIMARY KEY ("transactionId", "userId"));
CREATE INDEX "IDX_c46d3ce7d6d478fdb301fb6dc6" ON "transaction_participants_user" ("transactionId") ;
CREATE INDEX "IDX_1403966580332ae7a2da2321f4" ON "transaction_participants_user" ("userId") ;
ALTER TABLE "transaction" ADD CONSTRAINT "FK_8639071696032d9f59b2623bd25" FOREIGN KEY ("tripId") REFERENCES "trip"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "transaction" ADD CONSTRAINT "FK_e32e0d2862e47419a5bb7370787" FOREIGN KEY ("payerId") REFERENCES "user"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "transaction_participants_user" ADD CONSTRAINT "FK_c46d3ce7d6d478fdb301fb6dc66" FOREIGN KEY ("transactionId") REFERENCES "transaction"("id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "transaction_participants_user" ADD CONSTRAINT "FK_1403966580332ae7a2da2321f47" FOREIGN KEY ("userId") REFERENCES "user"("id") ON DELETE CASCADE ON UPDATE CASCADE;