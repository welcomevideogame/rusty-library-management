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
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import CheckCircleIcon from '@mui/icons-material/CheckCircle';
import CancelIcon from '@mui/icons-material/Cancel';
import ArrowUpwardIcon from '@mui/icons-material/ArrowUpward';
import ArrowDownwardIcon from '@mui/icons-material/ArrowDownward';
import Checkbox from '@mui/material/Checkbox';

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';

export default function Content() {
  const [tabValue, setTabValue] = React.useState(0);
  const [search, setSearch] = React.useState("");
  const [mediaData, setMediaData] = React.useState([]);
  const [allMediaData, setAllMediaData] = React.useState([]);
  const [sortConfig, setSortConfig] = useState({ key: '', direction: 'ascending' });
  const [checkedItems, setCheckedItems] = useState({});
  const [checkoutData, setCheckoutData] = useState([]);

  const tableCellStyle = {
    borderLeft: '1px solid rgba(224, 224, 224, 1)',
    borderRight: '1px solid rgba(224, 224, 224, 1)'
  };

  const handleTabChange = (event, newValue) => {
    setTabValue(newValue);
  };

  useEffect(() => {
    const selectedItems = allMediaData.filter(media => checkedItems[media.id]);
    setCheckoutData(selectedItems);
  }, [checkedItems, allMediaData]);

  const handleCheckboxChange = (id) => {
    setCheckedItems(prevState => ({
      ...prevState,
      [id]: !prevState[id]
    }));
  };

  async function searchMedia() {
    await invoke('search_media', { search: search })
    .then((json) => {
      const data = JSON.parse(json);
      setMediaData(data);
    })
    .catch((error) => console.error('Error fetching media data:', error));
  }

  async function getAllMedia() {
    await invoke('get_media')
    .then((json) => {
        const data = JSON.parse(json);
        setAllMediaData(data);
        const newCheckedItems = {};
        data.forEach(item => {
          newCheckedItems[item.id] = false; // Initialize all as unchecked
        });
        setCheckedItems(newCheckedItems);
    })
    .catch((error) => console.error('Error fetching media data:', error));
  }
  
  useEffect(() => {
    if (tabValue === 1) {
      getAllMedia();
    }
  }, [tabValue]);

  const renderMediaItem = (media) => {
    return (
      <Paper sx={{ my: 2, mx: 2, p: 2 }} key={media.id}>
        <Typography variant="h6">{media.name}</Typography>
        <Typography color="text.secondary">Type: {media.media_type}</Typography>
        <Typography color="text.secondary">Vendor: {media.vendor}</Typography>
        <Typography color="text.secondary">{media.borrowable ? 'Borrowable' : 'Not Borrowable'}</Typography>
        <Typography color="text.secondary">Rented by: {media.renter}</Typography>
      </Paper>
    );
  };

  const renderBorrowableIcon = (borrowable) => {
    return borrowable ? (
      <CheckCircleIcon style={{ color: 'green' }} />
    ) : (
      <CancelIcon style={{ color: 'red' }} />
    );
  };

  const requestSort = (key) => {
    let direction = 'ascending';
    if (sortConfig.key === key && sortConfig.direction === 'ascending') {
      direction = 'descending';
    }
    setSortConfig({ key, direction });
  }

  const sortedMediaData = React.useMemo(() => {
    let sortableItems = [...allMediaData];
    if (sortConfig !== null) {
      sortableItems.sort((a, b) => {
        if (a[sortConfig.key] < b[sortConfig.key]) {
          return sortConfig.direction === 'ascending' ? -1 : 1;
        }
        if (a[sortConfig.key] > b[sortConfig.key]) {
          return sortConfig.direction === 'ascending' ? 1 : -1;
        }
        return 0;
      });
    }
    return sortableItems;
  }, [allMediaData, sortConfig]);

  const getSortDirectionIcon = (columnName) => {
    if (sortConfig.key === columnName) {
      return sortConfig.direction === 'ascending' ? <ArrowUpwardIcon /> : <ArrowDownwardIcon />;
    }
    return null;
  };

  async function handleCheckout() {
    const jsonData = JSON.stringify(checkoutData);
    await invoke('media_checkout', { cart: jsonData })
    .then((json) => {
      const data = JSON.parse(json);
      setMediaData(data);
    })
    .catch((error) => console.error('Error fetching media data:', error));
  }

  const renderCheckoutContent = () => {
    return (
      <Paper sx={{ maxWidth: 1000, margin: 'auto', p: 2 }}>
        <Typography variant="h6" sx={{ mb: 2 }}>Items</Typography>
          {checkoutData.map(item => (
            <Typography key={item.id}>
              {item.name} - {item.media_type}
            </Typography>
          ))}
          <Button
            variant="contained"
            sx={{ mr: 1 }}
            onClick={handleCheckout}>
            Search
          </Button>
      </Paper>
    );
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
                      placeholder="Search by title"
                      InputProps={{
                        disableUnderline: true,
                        sx: { fontSize: 'default' },
                      }}
                      variant="standard"
                      onChange={(e) => setSearch(e.currentTarget.value)}
                    />
                  </Grid>
                  <Grid item>
                    <Button
                      variant="contained"
                      sx={{ mr: 1 }}
                      onClick={searchMedia}>
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
            {mediaData.length === 0 ? (
            <Typography sx={{ my: 5, mx: 2 }} color="text.secondary" align="center">
              Nothing found
            </Typography>
              ) : (
                <div>
                {mediaData.map((media) => renderMediaItem(media))}
              </div>
            )}
          </Paper>
        );
      case 1:
        return (
          <TableContainer component={Paper}>
            <Table sx={{ minWidth: 650 }} aria-label="media table">
              <TableHead>
                <TableRow>
                  <TableCell sx={tableCellStyle} onClick={() => requestSort('name')}>
                    Name {getSortDirectionIcon('name')}
                  </TableCell>
                  <TableCell sx={tableCellStyle} align="right" onClick={() => requestSort('media_type')}>
                    Type {getSortDirectionIcon('media_type')}
                  </TableCell>
                  <TableCell sx={tableCellStyle} align="right" onClick={() => requestSort('vendor')}>
                    Vendor {getSortDirectionIcon('vendor')}
                  </TableCell>
                  <TableCell sx={tableCellStyle} align="right" onClick={() => requestSort('borrowable')}>
                    Borrowable {getSortDirectionIcon('borrowable')}
                  </TableCell>
                  <TableCell sx={tableCellStyle} align="right" onClick={() => requestSort('renter')}>
                    Rented By {getSortDirectionIcon('renter')}
                  </TableCell>
                  <TableCell sx={tableCellStyle}>Select</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {sortedMediaData.map((media) => (
                    <TableRow key={media.id}>
                    <TableCell sx={tableCellStyle} component="th" scope="row">{media.name}</TableCell>
                    <TableCell sx={tableCellStyle} align="right">{media.media_type}</TableCell>
                    <TableCell sx={tableCellStyle} align="right">{media.vendor}</TableCell>
                    <TableCell sx={tableCellStyle} align="right">{renderBorrowableIcon(media.borrowable)}</TableCell>
                    <TableCell sx={tableCellStyle} align="right">{media.renter}</TableCell>
                    <TableCell sx={tableCellStyle} align="right">
                      <Checkbox
                        disabled={!media.borrowable}
                        checked={checkedItems[media.id] || false}
                        onChange={() => handleCheckboxChange(media.id)}
                      />
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        );
      case 2:
        return renderCheckoutContent();
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
        <Tab label="Reserve" />
      </Tabs>
      <br></br>
      {renderTabContent()}
    </Box>
  );
}
