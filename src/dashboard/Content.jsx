import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Paper from '@mui/material/Paper';
import Grid from '@mui/material/Grid';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Tooltip from '@mui/material/Tooltip';
import IconButton from '@mui/material/IconButton';
import SearchIcon from '@mui/icons-material/Search';
import RefreshIcon from '@mui/icons-material/Refresh';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';

export default function Content() {
  const [tabValue, setTabValue] = React.useState(0);

  const handleTabChange = (event, newValue) => {
    setTabValue(newValue);
  };

  const renderTabContent = () => {
    switch (tabValue) {
      case 0:
        return (
          <Paper sx={{ maxWidth: 1000, margin: 'auto', overflow: 'hidden' }}>
            <AppBar
              position="static"
              color="default"
              elevation={0}
              sx={{ borderBottom: '1px solid rgba(0, 0, 0, 0.12)' }}
            >
              <Toolbar>
                <Grid container spacing={2} alignItems="center">
                  <Grid item>
                    <SearchIcon color="inherit" sx={{ display: 'block' }} />
                  </Grid>
                  <Grid item xs>
                    <TextField
                      fullWidth
                      placeholder="Search by title, publisher, etc"
                      InputProps={{
                        disableUnderline: true,
                        sx: { fontSize: 'default' },
                      }}
                      variant="standard"
                    />
                  </Grid>
                  <Grid item>
                    <Button variant="contained" sx={{ mr: 1 }}>
                      Search
                    </Button>
                    <Tooltip title="Clear">
                      <IconButton>
                        <RefreshIcon color="inherit" sx={{ display: 'block' }} />
                      </IconButton>
                    </Tooltip>
                  </Grid>
                </Grid>
              </Toolbar>
            </AppBar>
            <Typography sx={{ my: 5, mx: 2 }} color="text.secondary" align="center">
              No users for this project yet
            </Typography>
          </Paper>
        );
      case 1:
        return (
          <Typography sx={{ my: 5, mx: 2 }} color="text.secondary" align="center">
            Content for Tab 2
          </Typography>
        );
      case 2:
        return (
          <Typography sx={{ my: 5, mx: 2 }} color="text.secondary" align="center">
            Content for Tab 3
          </Typography>
        );
      default:
        return null;
    }
  };

  return (
    <Box>
      <Tabs
        value={tabValue}
        onChange={handleTabChange}
        textColor="primary"
        centered
        style={{ border: 'none' }}
        TabIndicatorProps={{
          style: {
            backgroundColor: 'black',
          },
        }}
      >
        <Tab label="Search" style={{ accentColor: "black" }} /> 
        <Tab label="Full View" />
        <Tab label="Placeholder" />
      </Tabs>
      <br></br>
      {renderTabContent()}
    </Box>
  );
}
