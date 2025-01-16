#include <stdio.h>


#define NUM_DATA_POINTS 100
#define I2C_ADDRESS 0x84             // Default address for EVK-M101 module 


/**
 * @brief Struct to store the number of available satelites for each possible constellation, used
 * which is possible to get by parsing the `GSV` message.
 *
 */
typedef struct AVAILABLE_SATELITES_TABLE {
    unsigned char GP;
    unsigned char GL;
    unsigned char GA;
    unsigned char GB;
    unsigned char GI;
    unsigned char GQ;
} available_satelites_table;

/**
 * @brief Base struct for all numerical measurement from the GNSS module, containing
 * relevant metadata.
 *
 */
typedef struct GNSS_NUMERIC_MEASUREMENT {
    char is_available;         // Check if measurement was available in the last reading
    double value;              // Last available values
    char unit_of_measurement;  // Engineering Unit of measurement
} gnss_numeric_measurement;

/**
 * @brief Struct containing UTC data time information, and relevant metadata.
 *
 */
typedef struct UTC_DATE_TIME {
    unsigned char year;
    unsigned char month;
    unsigned char day;
    unsigned char hour;
    unsigned char minute;
    float second;

    char is_available;
} utc_date_time;

/**
 * @brief Struct to hold data from both latitude and longitude.
 *
 */
typedef struct GNSS_LAT_LONG_MEASUREMENT {
    char is_available;
    int degrees;
    float minutes;
    char indicator;
} gnss_lat_long_measurement;

/**
 * @brief Struct with all the necessary data for the working of the GNSS module as well as its readings.
 *
 */
typedef struct M10_GNSS {
    available_satelites_table num_available_satelites;
    gnss_lat_long_measurement latitude;
    gnss_lat_long_measurement longitude;
    gnss_numeric_measurement course_over_ground;
    gnss_numeric_measurement speed_over_ground_knots;
    utc_date_time time_of_sample;
    char buffer_empty;

    int i2c_handle;
    int i2c_address;
} m10_gnss;

int main(void) {
    m10_gnss data_points[NUM_DATA_POINTS];

    for (int i = 0; i < NUM_DATA_POINTS; i++) {

            data_points[i] = (m10_gnss){
                .num_available_satelites = {
                    // .GP = 0 + i,
                    .GP = 8,
                    .GL = 1 + i,
                    .GA = 2 + i,
                    .GB = 0,
                    .GI = 3 + i,
                    .GQ = 0
                },
                .latitude = {
                    .is_available = i%2,
                    .degrees = i,
                    .minutes = 0.1 + i,
                    .indicator = (i%2)?'N':'S'
                },
                .longitude = {
                    .is_available = i%2,
                    .degrees = i,
                    .minutes = 0.1 + i,
                    .indicator = (i%2)?'E':'W'
                },
                .course_over_ground = {
                    .is_available = i%2,
                    .value = 0.1+i,
                    .unit_of_measurement = 'm'
                },
                .speed_over_ground_knots = {
                    .is_available = i%2,
                    .value = 0.2+i,
                    .unit_of_measurement = 'k'
                },
                .time_of_sample = {
                    .year = 25,
                    .month = 1,
                    .day = 13,
                    .hour = 11,
                    .minute = i%60,
                    .second = 20,
                    .is_available=i%3
                },
                .buffer_empty = 0,
                .i2c_handle = 0x0,
                .i2c_address = 0x0
                    };
    }

	FILE* file;
    file = fopen("data_dump.gnss", "wb");

    if (file == NULL){
        perror("Unable to open file");
        return 1;
    }

    fwrite(data_points, sizeof(m10_gnss), NUM_DATA_POINTS, file);
 
}