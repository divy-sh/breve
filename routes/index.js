const express = require('express');
const router = express.Router();

// GET /homepage
router.get('/home', async (req, res) => {
  res.send('It works!');
});

module.exports = router;