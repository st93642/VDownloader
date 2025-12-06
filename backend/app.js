const express = require("express");
const cors = require("cors");
const applyRoutes = require("./routes");

const app = express();

app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

applyRoutes(app);

module.exports = app;
