import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Button, TextField, Container, Typography, Box, Grid, Paper } from "@mui/material";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [pdfFiles, setPdfFiles] = useState<File[]>([]);
  const [password, setPassword] = useState("");
  const [outputFileName, setOutputFileName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  const handleEncrypt = async () => {
    const fileNames = pdfFiles.map((file) => file.name);
    await invoke("encrypt_pdfs", { inputFiles: fileNames, password });
  };

  const handleDecrypt = async () => {
    const fileNames = pdfFiles.map((file) => file.name);
    await invoke("decrypt_pdfs", { inputFiles: fileNames, password });
  };

  const handleMerge = async () => {
    const fileNames = pdfFiles.map((file) => file.name);
    await invoke("merge_pdfs", { inputFiles: fileNames, outputFile: outputFileName });
  };

  return (
    <Container>
      <Typography variant="h4" gutterBottom>
        PDF Master Rust
      </Typography>
      <Box>
        <input
          type="file"
          multiple
          onChange={(e) => setPdfFiles(Array.from(e.target.files || []))}
        />
        <TextField
          label="Password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          fullWidth
          margin="normal"
        />
        <TextField
          label="Output File Name"
          value={outputFileName}
          onChange={(e) => setOutputFileName(e.target.value)}
          fullWidth
          margin="normal"
        />
        <Grid container spacing={2}>
          <Grid item>
            <Button variant="contained" color="primary" onClick={handleEncrypt}>
              Encrypt PDFs
            </Button>
          </Grid>
          <Grid item>
            <Button variant="contained" color="secondary" onClick={handleDecrypt}>
              Decrypt PDFs
            </Button>
          </Grid>
          <Grid item>
            <Button variant="contained" color="default" onClick={handleMerge}>
              Merge PDFs
            </Button>
          </Grid>
        </Grid>
      </Box>
    </Container>
  );
}

export default App;
