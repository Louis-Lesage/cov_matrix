use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};

pub fn get_cov_matrix_from_csv(file_name: &str) -> Result<Vec<Vec<f32>>, Error> {
    let file = File::open(file_name)?;
    let mut f = BufReader::new(file);
    let line = &mut String::new();

    f.read_line(line).unwrap();
    let dim_count = line.split(",").collect::<Vec<_>>().len();

    //diagonal of covariance matrix
    let (means, mult_means) = get_expected_values(Box::new(f), dim_count)?;

    let mut matrix = vec![vec![0.0; dim_count]; dim_count];

    for (i, line) in mult_means.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            matrix[i][j] = val - (means[i] * means[j]);
        }
    }

    Ok(matrix)
}

//return of each variables
//when E(x,y) ~= E(x)E(Y) see https://en.wikipedia.org/wiki/Covariance#Numerical_computation
fn get_expected_values(buf: Box<dyn BufRead>, dim_count: usize) -> Result<(Vec<f32>, Vec<Vec<f32>>), Error> {
    let mut sums: Vec<f32> = vec![0.0; dim_count];
    let mut mult_sums: Vec<Vec<f32>> = vec![vec![0.0; dim_count]; dim_count];
    let mut line_count = 0;

    for line in buf.lines() {
        line_count += 1;

        let line_val = line.unwrap();
        let line_values: Vec<&str> = line_val.split(",").collect::<Vec<&str>>();

        if line_values.len() > dim_count {
            return Err(Error::new(ErrorKind::InvalidInput, "Amount of values different than header"));
        }

        line_values.iter().enumerate().for_each(|(i, s)| {
            let i_val = s.parse::<f32>().unwrap();
            sums[i] += i_val;

            line_values.iter().enumerate().for_each(|(j, f)| {
                let j_val = f.parse::<f32>().unwrap();

                mult_sums[i][j] += i_val * j_val;
            });
        });
    }

    //avoid 0 division
    if line_count == 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "No values found"));
    }

    let x = sums.iter().map(|v| v / (line_count as f32)).collect();
    let y: Vec<Vec<f32>> = mult_sums.iter()
        .map(|line| {
            line.iter()
                .map(|v| v / line_count as f32)
                .collect()
        })
        .collect();
    Ok((x, y))
}

fn matrix_to_csv(file: File, matrix: Vec<Vec<f32>>) -> Result<(), Error> {
    //todo : make it work
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::get_cov_matrix_from_csv;

    #[test]
    fn test_get_cov_matrix_from_csv() {
        assert_eq!(get_cov_matrix_from_csv("test.csv").unwrap(),
                   vec![vec![8.0 / 3.0, 2.0 / 3.0], vec![2.0 / 3.0, 2.0 / 3.0]]);


        //todo : more tests
    }
}
