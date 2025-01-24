const form = document.getElementById('upload-form');
const status = document.getElementById('status');

form.addEventListener('submit', async (event) => {
  event.preventDefault();

  const fileInput = document.getElementById('file-input');
  const file = fileInput.files[0];

  const numberInput = document.getElementById('number');
  const number = numberInput.value;

  const formData = new FormData();
  formData.append('file', file);

  try {
    const reponse = await fetch('/upload', {
      method: 'POST',
      body: formData,
      headers: {
        'Number': `${number}`,
      }
    });

    if (reponse.ok) {
      status.textContent = 'File uploaded successfully!';
    } else {
      status.textContent = `Failed to upload file: ${reponse.statusText}`;
    }
  } catch (error) {
    status.textContent = `Error: ${error.message}`;
  }
});


